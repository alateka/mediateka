migrate:
	diesel setup --database-url mediateka.db && diesel migration run --database-url mediateka.db