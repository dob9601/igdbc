export DATABASE_URL := "sqlite://igdbc.db"

regenerate-entities:
    sea-orm-cli migrate fresh
    sea-orm-cli generate entity \
        --output-dir {{justfile_directory()}}/src/models/_entities/

generate-migration migration_name:
    sea-orm-cli migrate generate {{migration_name}}
