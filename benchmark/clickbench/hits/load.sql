COPY INTO hits
FROM 'https://datasets.bigbytesdb.org/hits_100m_obfuscated_v1.tsv.xz' FILE_FORMAT = (
        type = TSV compression = XZ field_delimiter = '\t' record_delimiter = '\n' skip_header = 0
    );
ANALYZE TABLE hits;
