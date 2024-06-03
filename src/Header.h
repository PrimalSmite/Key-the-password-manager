#pragma once

#include <iostream>
#include <string>
#include <time.h>
#include <Windows.h>
#include "sqlite/sqlite3.h"

int Data(string a, string b, string c);
void Print_all();
int Print(string name);
int callback(void* data, int argc, char** argv, char** azColName);
int Change_password(string _name, string new_data);
int Change_login(string _name, string new_data);
int Remove_data(string name);
