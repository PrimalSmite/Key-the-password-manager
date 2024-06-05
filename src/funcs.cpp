#include "Header.h"

//creating a password
string Generate_password(int count) {
	//array of symbols
	const int ch_MAX = 67;
	const char symbols[ch_MAX] = { 'a', 'b', 'c', 'd', 'e', 'f', 'g',
							  'h', 'i', 'j', 'k', 'l', 'm', 'n',
							  'o', 'p', 'q', 'r', 's', 't', 'u',
							  'v', 'w', 'x', 'y', 'z', 'A','B',
							  'C','D','E','F','G','H','I','J',
							  'K','l','M','N','O','P','Q','R',
							  'S','T','U','V','W','X','Y','Z',
							  '0','1','2','3','4','5','6','7',
							  '8','9','!','@','#','%','?' };

	string result_password = "";
	for (int i = 0; i < count; i++)
	{
		result_password = result_password + symbols[rand() % ch_MAX];
	}
	cout << result_password << endl;
	return result_password;
}

//saving data
int Data(string a, string b, string c) {
	sqlite3* db;
	char* errMsg = 0;

	//connecting to the database
	int rc = sqlite3_open("Passwords.db", &db);
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
	int rc = sqlite3_open("Passwords.db", &db);
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
	int rc = sqlite3_open("Passwords.db", &db);
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
	int rc = sqlite3_open("Passwords.db", &db);
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
	int rc = sqlite3_open("Passwords.db", &db);
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
	int rc = sqlite3_open("Passwords.db", &db);
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
