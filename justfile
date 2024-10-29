generate-entities:
    sea-orm-cli generate entity \
        --database-url sqlite:///{{justfile_directory()}}/api/igdbc.db \
        --output-dir {{justfile_directory()}}/api/src/models/_entities/
