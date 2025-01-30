<h1 align="center">Bigbytes: The Next-Gen Cloud [Data+AI] Analytics</h1>
<h2 align="center">The open-source, on-premise alternative to Snowflake</h2>


## 🐋 Introduction

**Bigbytes**, built in Rust, is an open-source cloud data warehouse that serves as a cost-effective [alternative to Snowflake](https://github.com/getbigbytes/bigbytes/issues/13059). With its focus on fast query execution and data ingestion, it's designed for complex analysis of the world's largest datasets.

**Production-Proven Scale:**
- 🤝 **Enterprise Adoption**: Trusted by over **50 organizations** processing more than **100 million queries daily**
- 🗄️ **Massive Scale**: Successfully managing over **800 petabytes** of analytical data

## ⚡ Performance

<div align="center">

[TPC-H Benchmark: Bigbytes Cloud vs. Snowflake](https://docs.bigbytes.com/guides/benchmark/tpch)

</div>

![Bigbytes vs. Snowflake](https://github.com/getbigbytes/wizard/assets/172204/d796acf0-0a66-4b1d-8754-cd2cd1de04c7)

<div align="center">

[Data Ingestion Benchmark: Bigbytes Cloud vs. Snowflake](https://docs.bigbytes.com/guides/benchmark/data-ingest)

</div>

![Bigbytes vs. Snowflake](https://github.com/getbigbytes/bigbytes/assets/172204/c61d7a40-f6fe-4fb9-83e8-06ea9599aeb4)


## 🚀 Why Bigbytes

- **Full Control**: Deploy on **cloud** or **on-prem** to suit your needs.

- **Blazing-Fast Performance**: Built with **Rust** for high-speed query execution. 👉 [ClickBench](https://bigbytes.com/blog/clickbench-bigbytes-top)

- **Cost-Effective**: Scalable architecture that boosts **performance** and reduces **costs**. 👉 [TPC-H](https://docs.bigbytes.com/guides/benchmark/tpch)

- **AI-Enhanced Analytics**: Leverage built-in **[AI Functions](https://docs.bigbytes.com/guides/ai-functions/)** for smarter data insights.

- **Simplified ETL**: Direct **data ingestion** without the need for external ETL tools. 👉 [Data Loading](https://docs.bigbytes.com/guides/load-data/)

- **Real-Time Data Updates**: Keep your analytics **up-to-date** with real-time incremental data updates. 👉 [Stream](https://docs.bigbytes.com/guides/load-data/continuous-data-pipelines/stream)

- **Advanced Indexing**: Boost query performance with **[Virtual Column](https://docs.bigbytes.com/guides/performance/virtual-column)**, **[Aggregating Index](https://docs.bigbytes.com/guides/performance/aggregating-index)**, and **[Full-Text Index](https://docs.bigbytes.com/guides/performance/fulltext-index)**.

- **ACID Compliance + Version Control**: Ensure reliable **transactions** with full ACID compliance and Git-like versioning.

- **Schema Flexibility**: Effortlessly handle **semi-structured data** with the flexible **[VARIANT](https://docs.bigbytes.com/sql/sql-reference/data-types/variant)** data type.

- **Community-Driven Growth**: **Open-source** and continuously evolving with contributions from a global community.



## 📐 Architecture

![Bigbytes Architecture](https://github.com/getbigbytes/bigbytes/assets/172204/68b1adc6-0ec1-41d4-9e1d-37b80ce0e5ef)

## 🚀 Try Bigbytes

### 1. Bigbytes Serverless Cloud

The fastest way to try Bigbytes, [Bigbytes Cloud](https://bigbytes.com)

### 2. Install Bigbytes from Docker

Prepare the image (once) from Docker Hub (this will download about 170 MB data):

```shell
docker pull getbigbytes/bigbytes
```

To run Bigbytes quickly:

```shell
docker run --net=host  getbigbytes/bigbytes
```

## 🚀 Getting Started

<details>
<summary>Connecting to Bigbytes</summary>

- [Connecting to Bigbytes with BendSQL](https://docs.bigbytes.com/guides/sql-clients/bendsql)
- [Connecting to Bigbytes with JDBC](https://docs.bigbytes.com/guides/sql-clients/jdbc)

</details>

<details>
<summary>Data Import and Export</summary>

- [How to load Parquet file into a table](https://docs.bigbytes.com/guides/load-data/load-semistructured/load-parquet)
- [How to export a table to Parquet file](https://docs.bigbytes.com/guides/unload-data/unload-parquet)
- [How to load CSV file into a table](https://docs.bigbytes.com/guides/load-data/load-semistructured/load-csv)
- [How to export a table to CSV file](https://docs.bigbytes.com/guides/unload-data/unload-csv)
- [How to load TSV file into a table](https://docs.bigbytes.com/guides/load-data/load-semistructured/load-tsv)
- [How to export a table to TSV file](https://docs.bigbytes.com/guides/unload-data/unload-tsv)
- [How to load NDJSON file into a table](https://docs.bigbytes.com/guides/load-data/load-semistructured/load-ndjson)
- [How to export a table to NDJSON file](https://docs.bigbytes.com/guides/unload-data/unload-ndjson)
- [How to load ORC file into a table](https://docs.bigbytes.com/guides/load-data/load-semistructured/load-orc)

</details>

<details>
<summary>Loading Data From Other Databases</summary>

- [How to Sync Full and Incremental MySQL Changes into Bigbytes](https://docs.bigbytes.com/guides/load-data/load-db/debezium)
- [How to Sync Full and Incremental PostgreSQL Changes into Bigbytes](https://docs.bigbytes.com/guides/load-data/load-db/flink-cdc)
- [How to Sync Full and Incremental Oracle Changes into Bigbytes](https://docs.bigbytes.com/guides/load-data/load-db/flink-cdc)

</details>

<details>
<summary>Querying Semi-structured Data</summary>

- [How to query directly on Parquet file](https://docs.bigbytes.com/guides/load-data/transform/querying-parquet)
- [How to query directly on CSV file](https://docs.bigbytes.com/guides/load-data/transform/querying-csv)
- [How to query directly on TSV file](https://docs.bigbytes.com/guides/load-data/transform/querying-tsv)
- [How to query directly on NDJSON file](https://docs.bigbytes.com/guides/load-data/transform/querying-ndjson)
- [How to query directly on ORC file](https://docs.bigbytes.com/guides/load-data/transform/querying-orc)
</details>

<details>
<summary>Visualize Tools with Bigbytes</summary>

- [Deepnote](https://docs.bigbytes.com/guides/visualize/deepnote)
- [Grafana](https://docs.bigbytes.com/guides/visualize/grafana)
- [Jupyter Notebook](https://docs.bigbytes.com/guides/visualize/jupyter)
- [Metabase](https://docs.bigbytes.com/guides/visualize/metabase)
- [MindsDB](https://docs.bigbytes.com/guides/visualize/mindsdb)
- [Redash](https://docs.bigbytes.com/guides/visualize/redash)
- [Superset](https://docs.bigbytes.com/guides/visualize/superset)
- [Tableau](https://docs.bigbytes.com/guides/visualize/tableau)

</details>

<details>
<summary>Managing Users</summary>

- [How to Create a User](https://docs.bigbytes.com/sql/sql-commands/ddl/user/user-create-user)
- [How to Grant Privileges to a User](https://docs.bigbytes.com/sql/sql-commands/ddl/user/grant#granting-privileges)
- [How to Revoke Privileges from a User](https://docs.bigbytes.com/sql/sql-commands/ddl/user/revoke#revoking-privileges)
- [How to Create a Role](https://docs.bigbytes.com/sql/sql-commands/ddl/user/user-create-role)
- [How to Grant Privileges to a Role](https://docs.bigbytes.com/sql/sql-commands/ddl/user/grant#granting-role)
- [How to Grant Role to a User](https://docs.bigbytes.com/sql/sql-commands/ddl/user/grant)
- [How to Revoke the Role of a User](https://docs.bigbytes.com/sql/sql-commands/ddl/user/revoke#revoking-role)
</details>

<details>
<summary>Managing Databases</summary>

- [How to Create a Database](https://docs.bigbytes.com/sql/sql-commands/ddl/database/ddl-create-database)
- [How to Drop a Database](https://docs.bigbytes.com/sql/sql-commands/ddl/database/ddl-drop-database)
</details>

<details>
<summary>Managing Tables</summary>

- [How to Create a Table](https://docs.bigbytes.com/sql/sql-commands/ddl/table/ddl-create-table)
- [How to Drop a Table](https://docs.bigbytes.com/sql/sql-commands/ddl/table/ddl-drop-table)
- [How to Rename a Table](https://docs.bigbytes.com/sql/sql-commands/ddl/table/ddl-rename-table)
- [How to Truncate a Table](https://docs.bigbytes.com/sql/sql-commands/ddl/table/ddl-truncate-table)
- [How to Flash Back a Table](https://docs.bigbytes.com/sql/sql-commands/ddl/table/flashback-table)
- [How to Add/Drop Table Column](https://docs.bigbytes.com/sql/sql-commands/ddl/table/alter-table-column)
</details>

<details>
<summary>Managing Data</summary>

- [COPY-INTO](https://docs.bigbytes.com/sql/sql-commands/dml/dml-copy-into-table)
- [INSERT](https://docs.bigbytes.com/sql/sql-commands/dml/dml-insert)
- [DELETE](https://docs.bigbytes.com/sql/sql-commands/dml/dml-delete-from)
- [UPDATE](https://docs.bigbytes.com/sql/sql-commands/dml/dml-update)
- [REPLACE](https://docs.bigbytes.com/sql/sql-commands/dml/dml-replace)
- [MERGE-INTO](https://docs.bigbytes.com/sql/sql-commands/dml/dml-merge)
</details>

<details>
<summary>Managing Views</summary>

- [How to Create a View](https://docs.bigbytes.com/sql/sql-commands/ddl/view/ddl-create-view)
- [How to Drop a View](https://docs.bigbytes.com/sql/sql-commands/ddl/view/ddl-drop-view)
- [How to Alter a View](https://docs.bigbytes.com/sql/sql-commands/ddl/view/ddl-alter-view)
</details>

<details>
<summary>AI Functions</summary>

- [Generating SQL with AI](https://docs.bigbytes.com/sql/sql-functions/ai-functions/ai-to-sql)
- [Creating Embedding Vectors](https://docs.bigbytes.com/sql/sql-functions/ai-functions/ai-embedding-vector)
- [Computing Text Similarities](https://docs.bigbytes.com/sql/sql-functions/ai-functions/ai-cosine-distance)
- [Text Completion with AI](https://docs.bigbytes.com/sql/sql-functions/ai-functions/ai-text-completion)
</details>

<details>
<summary>Data Management</summary>

- [Data Lifecycle in Bigbytes](https://docs.bigbytes.com/guides/data-management/data-lifecycle)
- [Data Recovery in Bigbytes](https://docs.bigbytes.com/guides/data-management/data-recovery)
- [Data Protection in Bigbytes](https://docs.bigbytes.com/guides/data-management/data-protection)
- [Data Purge in Bigbytes](https://docs.bigbytes.com/guides/data-management/data-recycle)

</details>

<details>
<summary>Accessing Data Lake</summary>

- [Apache Hive](https://docs.bigbytes.com/guides/access-data-lake/hive)
- [Apache Iceberg](https://docs.bigbytes.com/guides/access-data-lake/iceberg/iceberg-engine)
- [Delta Lake](https://docs.bigbytes.com/guides/access-data-lake/delta)

</details>

<details>
<summary>Security</summary>

- [Access Control](https://docs.bigbytes.com/guides/security/access-control)
- [Masking Policy](https://docs.bigbytes.com/guides/security/masking-policy)
- [Network Policy](https://docs.bigbytes.com/guides/security/network-policy)
- [Password Policy](https://docs.bigbytes.com/guides/security/password-policy)

</details>

<details>
<summary>Performance</summary>

- [Review Clickbench](https://bigbytes.com/blog/clickbench-bigbytes-top)
- [TPC-H Benchmark: Bigbytes Cloud vs. Snowflake](https://docs.bigbytes.com/guides/benchmark/tpch)
- [Bigbytes vs. Snowflake: Data Ingestion Benchmark](https://docs.bigbytes.com/guides/benchmark/data-ingest)

</details>

## 🤝 Contributing

Bigbytes thrives on community contributions! Whether it's through ideas, code, or documentation, every effort helps in enhancing our project. As a token of our appreciation, once your code is merged, your name will be eternally preserved in the **system.contributors** table.

Here are some resources to help you get started:

- [Building Bigbytes From Source](https://docs.bigbytes.com/guides/community/contributor/building-from-source)
- [The First Good Pull Request](https://docs.bigbytes.com/guides/community/contributor/good-pr)

## 👥 Community

For guidance on using Bigbytes, we recommend starting with the official documentation. If you need further assistance, explore the following community channels:

- [Slack](https://link.bigbytes.com/join-slack) (For live discussion with the Community)
- [GitHub](https://github.com/getbigbytes/bigbytes) (Feature/Bug reports, Contributions)
- [Twitter](https://twitter.com/getbigbytes/) (Get the news fast)
- [I'm feeling lucky](https://link.bigbytes.com/i-m-feeling-lucky) (Pick up a good first issue now!)
