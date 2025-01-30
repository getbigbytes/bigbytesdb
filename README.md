<h1 align="center">Bigbytesdb: The Next-Gen Cloud [Data+AI] Analytics</h1>
<h2 align="center">The open-source, on-premise alternative to Snowflake</h2>


## 🐋 Introduction

**Bigbytesdb**, built in Rust, is an open-source cloud data warehouse that serves as a cost-effective [alternative to Snowflake](https://github.com/getbigbytes/bigbytesdb/issues/13059). With its focus on fast query execution and data ingestion, it's designed for complex analysis of the world's largest datasets.

**Production-Proven Scale:**
- 🤝 **Enterprise Adoption**: Trusted by over **50 organizations** processing more than **100 million queries daily**
- 🗄️ **Massive Scale**: Successfully managing over **800 petabytes** of analytical data

## ⚡ Performance

<div align="center">

[TPC-H Benchmark: Bigbytesdb Cloud vs. Snowflake](https://docs.bigbytesdb.com/guides/benchmark/tpch)

</div>

![Bigbytesdb vs. Snowflake](https://github.com/getbigbytes/wizard/assets/172204/d796acf0-0a66-4b1d-8754-cd2cd1de04c7)

<div align="center">

[Data Ingestion Benchmark: Bigbytesdb Cloud vs. Snowflake](https://docs.bigbytesdb.com/guides/benchmark/data-ingest)

</div>

![Bigbytesdb vs. Snowflake](https://github.com/getbigbytes/bigbytesdb/assets/172204/c61d7a40-f6fe-4fb9-83e8-06ea9599aeb4)


## 🚀 Why Bigbytesdb

- **Full Control**: Deploy on **cloud** or **on-prem** to suit your needs.

- **Blazing-Fast Performance**: Built with **Rust** for high-speed query execution. 👉 [ClickBench](https://bigbytesdb.com/blog/clickbench-bigbytesdb-top)

- **Cost-Effective**: Scalable architecture that boosts **performance** and reduces **costs**. 👉 [TPC-H](https://docs.bigbytesdb.com/guides/benchmark/tpch)

- **AI-Enhanced Analytics**: Leverage built-in **[AI Functions](https://docs.bigbytesdb.com/guides/ai-functions/)** for smarter data insights.

- **Simplified ETL**: Direct **data ingestion** without the need for external ETL tools. 👉 [Data Loading](https://docs.bigbytesdb.com/guides/load-data/)

- **Real-Time Data Updates**: Keep your analytics **up-to-date** with real-time incremental data updates. 👉 [Stream](https://docs.bigbytesdb.com/guides/load-data/continuous-data-pipelines/stream)

- **Advanced Indexing**: Boost query performance with **[Virtual Column](https://docs.bigbytesdb.com/guides/performance/virtual-column)**, **[Aggregating Index](https://docs.bigbytesdb.com/guides/performance/aggregating-index)**, and **[Full-Text Index](https://docs.bigbytesdb.com/guides/performance/fulltext-index)**.

- **ACID Compliance + Version Control**: Ensure reliable **transactions** with full ACID compliance and Git-like versioning.

- **Schema Flexibility**: Effortlessly handle **semi-structured data** with the flexible **[VARIANT](https://docs.bigbytesdb.com/sql/sql-reference/data-types/variant)** data type.

- **Community-Driven Growth**: **Open-source** and continuously evolving with contributions from a global community.



## 📐 Architecture

![Bigbytesdb Architecture](https://github.com/getbigbytes/bigbytesdb/assets/172204/68b1adc6-0ec1-41d4-9e1d-37b80ce0e5ef)

## 🚀 Try Bigbytesdb

### 1. Bigbytesdb Serverless Cloud

The fastest way to try Bigbytesdb, [Bigbytesdb Cloud](https://bigbytesdb.com)

### 2. Install Bigbytesdb from Docker

Prepare the image (once) from Docker Hub (this will download about 170 MB data):

```shell
docker pull getbigbytes/bigbytesdb
```

To run Bigbytesdb quickly:

```shell
docker run --net=host  getbigbytes/bigbytesdb
```

## 🚀 Getting Started

<details>
<summary>Connecting to Bigbytesdb</summary>

- [Connecting to Bigbytesdb with BendSQL](https://docs.bigbytesdb.com/guides/sql-clients/bendsql)
- [Connecting to Bigbytesdb with JDBC](https://docs.bigbytesdb.com/guides/sql-clients/jdbc)

</details>

<details>
<summary>Data Import and Export</summary>

- [How to load Parquet file into a table](https://docs.bigbytesdb.com/guides/load-data/load-semistructured/load-parquet)
- [How to export a table to Parquet file](https://docs.bigbytesdb.com/guides/unload-data/unload-parquet)
- [How to load CSV file into a table](https://docs.bigbytesdb.com/guides/load-data/load-semistructured/load-csv)
- [How to export a table to CSV file](https://docs.bigbytesdb.com/guides/unload-data/unload-csv)
- [How to load TSV file into a table](https://docs.bigbytesdb.com/guides/load-data/load-semistructured/load-tsv)
- [How to export a table to TSV file](https://docs.bigbytesdb.com/guides/unload-data/unload-tsv)
- [How to load NDJSON file into a table](https://docs.bigbytesdb.com/guides/load-data/load-semistructured/load-ndjson)
- [How to export a table to NDJSON file](https://docs.bigbytesdb.com/guides/unload-data/unload-ndjson)
- [How to load ORC file into a table](https://docs.bigbytesdb.com/guides/load-data/load-semistructured/load-orc)

</details>

<details>
<summary>Loading Data From Other Databases</summary>

- [How to Sync Full and Incremental MySQL Changes into Bigbytesdb](https://docs.bigbytesdb.com/guides/load-data/load-db/debezium)
- [How to Sync Full and Incremental PostgreSQL Changes into Bigbytesdb](https://docs.bigbytesdb.com/guides/load-data/load-db/flink-cdc)
- [How to Sync Full and Incremental Oracle Changes into Bigbytesdb](https://docs.bigbytesdb.com/guides/load-data/load-db/flink-cdc)

</details>

<details>
<summary>Querying Semi-structured Data</summary>

- [How to query directly on Parquet file](https://docs.bigbytesdb.com/guides/load-data/transform/querying-parquet)
- [How to query directly on CSV file](https://docs.bigbytesdb.com/guides/load-data/transform/querying-csv)
- [How to query directly on TSV file](https://docs.bigbytesdb.com/guides/load-data/transform/querying-tsv)
- [How to query directly on NDJSON file](https://docs.bigbytesdb.com/guides/load-data/transform/querying-ndjson)
- [How to query directly on ORC file](https://docs.bigbytesdb.com/guides/load-data/transform/querying-orc)
</details>

<details>
<summary>Visualize Tools with Bigbytesdb</summary>

- [Deepnote](https://docs.bigbytesdb.com/guides/visualize/deepnote)
- [Grafana](https://docs.bigbytesdb.com/guides/visualize/grafana)
- [Jupyter Notebook](https://docs.bigbytesdb.com/guides/visualize/jupyter)
- [Metabase](https://docs.bigbytesdb.com/guides/visualize/metabase)
- [MindsDB](https://docs.bigbytesdb.com/guides/visualize/mindsdb)
- [Redash](https://docs.bigbytesdb.com/guides/visualize/redash)
- [Superset](https://docs.bigbytesdb.com/guides/visualize/superset)
- [Tableau](https://docs.bigbytesdb.com/guides/visualize/tableau)

</details>

<details>
<summary>Managing Users</summary>

- [How to Create a User](https://docs.bigbytesdb.com/sql/sql-commands/ddl/user/user-create-user)
- [How to Grant Privileges to a User](https://docs.bigbytesdb.com/sql/sql-commands/ddl/user/grant#granting-privileges)
- [How to Revoke Privileges from a User](https://docs.bigbytesdb.com/sql/sql-commands/ddl/user/revoke#revoking-privileges)
- [How to Create a Role](https://docs.bigbytesdb.com/sql/sql-commands/ddl/user/user-create-role)
- [How to Grant Privileges to a Role](https://docs.bigbytesdb.com/sql/sql-commands/ddl/user/grant#granting-role)
- [How to Grant Role to a User](https://docs.bigbytesdb.com/sql/sql-commands/ddl/user/grant)
- [How to Revoke the Role of a User](https://docs.bigbytesdb.com/sql/sql-commands/ddl/user/revoke#revoking-role)
</details>

<details>
<summary>Managing Databases</summary>

- [How to Create a Database](https://docs.bigbytesdb.com/sql/sql-commands/ddl/database/ddl-create-database)
- [How to Drop a Database](https://docs.bigbytesdb.com/sql/sql-commands/ddl/database/ddl-drop-database)
</details>

<details>
<summary>Managing Tables</summary>

- [How to Create a Table](https://docs.bigbytesdb.com/sql/sql-commands/ddl/table/ddl-create-table)
- [How to Drop a Table](https://docs.bigbytesdb.com/sql/sql-commands/ddl/table/ddl-drop-table)
- [How to Rename a Table](https://docs.bigbytesdb.com/sql/sql-commands/ddl/table/ddl-rename-table)
- [How to Truncate a Table](https://docs.bigbytesdb.com/sql/sql-commands/ddl/table/ddl-truncate-table)
- [How to Flash Back a Table](https://docs.bigbytesdb.com/sql/sql-commands/ddl/table/flashback-table)
- [How to Add/Drop Table Column](https://docs.bigbytesdb.com/sql/sql-commands/ddl/table/alter-table-column)
</details>

<details>
<summary>Managing Data</summary>

- [COPY-INTO](https://docs.bigbytesdb.com/sql/sql-commands/dml/dml-copy-into-table)
- [INSERT](https://docs.bigbytesdb.com/sql/sql-commands/dml/dml-insert)
- [DELETE](https://docs.bigbytesdb.com/sql/sql-commands/dml/dml-delete-from)
- [UPDATE](https://docs.bigbytesdb.com/sql/sql-commands/dml/dml-update)
- [REPLACE](https://docs.bigbytesdb.com/sql/sql-commands/dml/dml-replace)
- [MERGE-INTO](https://docs.bigbytesdb.com/sql/sql-commands/dml/dml-merge)
</details>

<details>
<summary>Managing Views</summary>

- [How to Create a View](https://docs.bigbytesdb.com/sql/sql-commands/ddl/view/ddl-create-view)
- [How to Drop a View](https://docs.bigbytesdb.com/sql/sql-commands/ddl/view/ddl-drop-view)
- [How to Alter a View](https://docs.bigbytesdb.com/sql/sql-commands/ddl/view/ddl-alter-view)
</details>

<details>
<summary>AI Functions</summary>

- [Generating SQL with AI](https://docs.bigbytesdb.com/sql/sql-functions/ai-functions/ai-to-sql)
- [Creating Embedding Vectors](https://docs.bigbytesdb.com/sql/sql-functions/ai-functions/ai-embedding-vector)
- [Computing Text Similarities](https://docs.bigbytesdb.com/sql/sql-functions/ai-functions/ai-cosine-distance)
- [Text Completion with AI](https://docs.bigbytesdb.com/sql/sql-functions/ai-functions/ai-text-completion)
</details>

<details>
<summary>Data Management</summary>

- [Data Lifecycle in Bigbytesdb](https://docs.bigbytesdb.com/guides/data-management/data-lifecycle)
- [Data Recovery in Bigbytesdb](https://docs.bigbytesdb.com/guides/data-management/data-recovery)
- [Data Protection in Bigbytesdb](https://docs.bigbytesdb.com/guides/data-management/data-protection)
- [Data Purge in Bigbytesdb](https://docs.bigbytesdb.com/guides/data-management/data-recycle)

</details>

<details>
<summary>Accessing Data Lake</summary>

- [Apache Hive](https://docs.bigbytesdb.com/guides/access-data-lake/hive)
- [Apache Iceberg](https://docs.bigbytesdb.com/guides/access-data-lake/iceberg/iceberg-engine)
- [Delta Lake](https://docs.bigbytesdb.com/guides/access-data-lake/delta)

</details>

<details>
<summary>Security</summary>

- [Access Control](https://docs.bigbytesdb.com/guides/security/access-control)
- [Masking Policy](https://docs.bigbytesdb.com/guides/security/masking-policy)
- [Network Policy](https://docs.bigbytesdb.com/guides/security/network-policy)
- [Password Policy](https://docs.bigbytesdb.com/guides/security/password-policy)

</details>

<details>
<summary>Performance</summary>

- [Review Clickbench](https://bigbytesdb.com/blog/clickbench-bigbytesdb-top)
- [TPC-H Benchmark: Bigbytesdb Cloud vs. Snowflake](https://docs.bigbytesdb.com/guides/benchmark/tpch)
- [Bigbytesdb vs. Snowflake: Data Ingestion Benchmark](https://docs.bigbytesdb.com/guides/benchmark/data-ingest)

</details>
