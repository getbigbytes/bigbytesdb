// Copyright 2024 Digitrans Inc
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use bigbytesdb_common_catalog::plan::PushDownInfo;
use bigbytesdb_common_catalog::table::Table;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_cloud_control::client_config::build_client_config;
use bigbytesdb_common_cloud_control::client_config::make_request;
use bigbytesdb_common_cloud_control::cloud_api::CloudControlApiProvider;
use bigbytesdb_common_cloud_control::notification_utils::NotificationParams;
use bigbytesdb_common_cloud_control::pb::ListNotificationRequest;
use bigbytesdb_common_cloud_control::pb::Notification;
use bigbytesdb_common_config::GlobalConfig;
use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::infer_table_schema;
use bigbytesdb_common_expression::types::BooleanType;
use bigbytesdb_common_expression::types::StringType;
use bigbytesdb_common_expression::types::TimestampType;
use bigbytesdb_common_expression::types::UInt64Type;
use bigbytesdb_common_expression::types::VariantType;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::FromData;
use bigbytesdb_common_meta_app::schema::TableIdent;
use bigbytesdb_common_meta_app::schema::TableInfo;
use bigbytesdb_common_meta_app::schema::TableMeta;
use bigbytesdb_common_sql::plans::notification_schema;

use crate::table::AsyncOneBlockSystemTable;
use crate::table::AsyncSystemTable;

pub fn parse_notifications_to_datablock(notification: Vec<Notification>) -> Result<DataBlock> {
    let mut created_on: Vec<i64> = Vec::with_capacity(notification.len());
    let mut name: Vec<String> = Vec::with_capacity(notification.len());
    let mut id: Vec<u64> = Vec::with_capacity(notification.len());
    let mut n_type: Vec<String> = Vec::with_capacity(notification.len());
    let mut enabled: Vec<bool> = Vec::with_capacity(notification.len());
    let mut webhook_options: Vec<Option<Vec<u8>>> = Vec::with_capacity(notification.len());
    let mut comment: Vec<Option<String>> = Vec::with_capacity(notification.len());

    for n in notification {
        let tsk: bigbytesdb_common_cloud_control::notification_utils::Notification = n.try_into()?;
        created_on.push(tsk.created_time.timestamp_micros());
        name.push(tsk.name);
        id.push(tsk.id);
        enabled.push(tsk.enabled);
        comment.push(tsk.comments);
        match tsk.params {
            NotificationParams::Webhook(opts) => {
                n_type.push("webhook".to_string());
                let serialized_webhook_options = serde_json::to_vec(&opts).unwrap();
                webhook_options.push(Some(serialized_webhook_options));
            }
        }
    }

    Ok(DataBlock::new_from_columns(vec![
        TimestampType::from_data(created_on),
        StringType::from_data(name),
        UInt64Type::from_data(id),
        StringType::from_data(n_type),
        BooleanType::from_data(enabled),
        VariantType::from_opt_data(webhook_options),
        StringType::from_opt_data(comment),
    ]))
}

pub struct NotificationsTable {
    table_info: TableInfo,
}

#[async_trait::async_trait]
impl AsyncSystemTable for NotificationsTable {
    const NAME: &'static str = "system.notifications";

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    #[async_backtrace::framed]
    async fn get_full_data(
        &self,
        ctx: Arc<dyn TableContext>,
        _push_downs: Option<PushDownInfo>,
    ) -> Result<DataBlock> {
        let config = GlobalConfig::instance();
        if config.query.cloud_control_grpc_server_address.is_none() {
            return Err(ErrorCode::CloudControlNotEnabled(
                "cannot view system.notifications table without cloud control enabled, please set cloud_control_grpc_server_address in config",
            ));
        }

        let tenant = ctx.get_tenant();
        let query_id = ctx.get_id();
        let user = ctx.get_current_user()?.identity().display().to_string();
        let req = ListNotificationRequest {
            tenant_id: tenant.tenant_name().to_string().clone(),
        };

        let cloud_api = CloudControlApiProvider::instance();
        let notification_client = cloud_api.get_notification_client();
        let mut cfg = build_client_config(
            tenant.tenant_name().to_string(),
            user,
            query_id,
            cloud_api.get_timeout(),
        );
        cfg.add_notification_version_info();
        let req = make_request(req, cfg);

        let resp = notification_client.list_notifications(req).await?;
        let notifications = resp.notifications;

        parse_notifications_to_datablock(notifications)
    }
}

impl NotificationsTable {
    pub fn create(table_id: u64) -> Arc<dyn Table> {
        let schema = infer_table_schema(&notification_schema())
            .expect("failed to parse notifications table schema");

        let table_info = TableInfo {
            desc: "'system'.'notifications'".to_string(),
            name: "notifications".to_string(),
            ident: TableIdent::new(table_id, 0),
            meta: TableMeta {
                schema,
                engine: "SystemNotifications".to_string(),

                ..Default::default()
            },
            ..Default::default()
        };

        AsyncOneBlockSystemTable::create(Self { table_info })
    }
}
