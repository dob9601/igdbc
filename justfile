export DATABASE_URL := "postgres://igdbc:password@172.18.0.4:5432/igdbc"

regenerate-entities:
    sea-orm-cli migrate fresh
    sea-orm-cli generate entity \
        --output-dir {{justfile_directory()}}/src/models/_entities/

generate-migration migration_name:
    sea-orm-cli migrate generate {{migration_name}}
