#include <iostream>
#include <string>
#include <time.h>
#include <Windows.h>
#include "sqlite/sqlite3.h"

//saving data
int saving(string a, string b, string c) {
	sqlite3* db;
	char* errMsg = 0;

	//connecting to the database
	int rc = sqlite3_open("Keys.db", &db);
	if (rc != SQLITE_OK) {
		cout << "Database is not opened!" << endl;
	}
	else {
		cout << "Database is opened!" << endl;

		//sql request to create a table passwords
		const char* passwords = "CREATE TABLE PASSWORDS("
			//"ID INTEGER PRIMARY KEY AUTOINCREMENT,"
			"NAME TEXT NOT NULL,"
			"LOGIN TEXT NOT NULL,"
			"PASSWORD TEXT NOT NULL);";
		rc = sqlite3_exec(db, passwords, 0, 0, &errMsg);
		if (rc != SQLITE_OK) {
			cout << "SQL error! May be the table is alredy used" << endl;
			sqlite3_free(errMsg);
		}
		else {
			cout << "Table created successfully!" << endl;
		}
	}
	//sql request to save the data
	string saving = "INSERT INTO PASSWORDS (NAME, LOGIN, PASSWORD) VALUES ('" + a + "', '" + b + "', '" + c + "');";
	rc = sqlite3_exec(db, saving.c_str(), 0, 0, &errMsg);
	if (rc != SQLITE_OK) {
		cout << "Error! Data is not saved" << endl;
		sqlite3_free(errMsg);
	}
	else {
		cout << "Success! Data is saved" << endl;
	}
	sqlite3_close(db);

	return 0;
}

int callback(void* data, int argc, char** argv, char** azColName) {
	int i;
	std::cout << "-----------------------------------" << std::endl;
	for (i = 0; i < argc; i++) {
		std::cout << azColName[i] << " = " << (argv[i] ? argv[i] : "NULL") << std::endl;
	}
	std::cout << "-----------------------------------" << std::endl;
	return 0;
}

//Printing all a data into the console
void Print_all() {
	sqlite3* db;
	char* errMsg = 0;
	int rc = sqlite3_open("Keys.db", &db);
	if (rc != SQLITE_OK) {
		cout << "Error! Cannot open the database" << endl;
	}
	else {
		//cout << "Database opened successful" << endl;


		const char* sql = "SELECT * FROM PASSWORDS";

		rc = sqlite3_exec(db, sql, callback, 0, &errMsg);
		if (rc != SQLITE_OK) {
			cout << "Error! Cannot read a data" << endl;
			sqlite3_free(errMsg);
		}
		sqlite3_close(db);
	}
}


//Printing a certain data into the console
int Print(string name) {
	sqlite3* db;
	char* errMsg = 0;
	int rc = sqlite3_open("Keys.db", &db);
	if (rc != SQLITE_OK) {
		cout << "Error! Cannot open the database" << endl;
	}
	else {
		//cout << "Database opened successful" << endl;


		string sql = "SELECT * FROM PASSWORDS WHERE NAME = '" + name + "';";

		rc = sqlite3_exec(db, sql.c_str(), callback, 0, &errMsg);
		if (rc != SQLITE_OK) {
			cout << "Error! Cannot read a data" << endl;
			sqlite3_free(errMsg);
		}
		sqlite3_close(db);
	}
	return 0;
}


//Change login
int Change_login(string _name, string new_data) {    //is nor working

	//connectring to the db
	sqlite3* db;
	char* errMsg = 0;
	int rc = sqlite3_open("Keys.db", &db);
	if (rc != SQLITE_OK) {
		cout << "Error! Cannot open the database" << endl;
	}
	else {
		cout << "Database opened successful" << endl;


		//creating sql request
		string sql = "UPDATE PASSWORDS SET LOGIN = '" + new_data + "' WHERE NAME = '" + _name + "';";
		rc = sqlite3_exec(db, sql.c_str(), 0, 0, &errMsg);
		if (rc != SQLITE_OK) {
			cout << "Error! Password was not changed. May be you entered a wrong values" << endl;
			sqlite3_free(errMsg);
		}
		else {
			cout << "Login changed successful" << endl;
		}
	}
	return 0;
}

//Change password
int Change_password(string _name, string new_data) { //is not working

	//connectring to the db
	sqlite3* db;
	char* errMsg = 0;
	int rc = sqlite3_open("Keys.db", &db);
	if (rc != SQLITE_OK) {
		cout << "Error! Cannot open the database" << endl;
	}
	else {
		cout << "Database opened successful" << endl;


		//creating sql request
		string sql = "UPDATE PASSWORDS SET PASSWORD = '" + new_data + "' WHERE NAME = '" + _name + "';";
		rc = sqlite3_exec(db, sql.c_str(), 0, 0, &errMsg);
		if (rc != SQLITE_OK) {
			cout << "Error! Password was not changed. May be you entered a wrong values" << endl;
			sqlite3_free(errMsg);
		}
		else {
			cout << "Password changed successful" << endl;
		}
	}
	return 0;
}

//Removing Data
int Remove_data(string name) {

	//connectring to the db
	sqlite3* db;
	char* errMsg = 0;
	int rc = sqlite3_open("Keys.db", &db);
	if (rc != SQLITE_OK) {
		cout << "Error! Cannot open the database" << endl;
	}
	else {
		cout << "Database opened successful" << endl;

		//creating a sql request
		string sql = "DELETE FROM PASSWORDS WHERE NAME = '" + name + "'";
		rc = sqlite3_exec(db, sql.c_str(), 0, 0, &errMsg);
		if (rc != SQLITE_OK) {
			cout << "Error! The data was not deleted! May be you entered a wrong service" << endl;
			sqlite3_free(db);
		}
		else {
			cout << "The data was deleted success!" << endl;
		}
		sqlite3_close(db);
	}
	return 0;
}
