#pragma once

#include <cstdlib>
#include <iostream>
#include <random>
#include <string>
#include <time.h>
// #include <Windows.h>
#include "sqlite/sqlite3.h"

using namespace std;

static int menu_act;

static string name, login, password;

static string warning =
    "If you did a mistake in input, just restart a program, a data, that you "
    "entered into a program will not save!";

static int symbols_count;
static int act;

static string opa = "You want to print all data (1), or only certain (2)?";

static string _name, new_data;

static string menu_one = "(1) Create a password";
static string menu_two = "(2) Save a password";
static string menu_tree = "(3) Show a password";
static string menu_four = "(4) Change a password";
static string menu_five = "(5) Change a login";
static string menu_six = "(6) Remove a password";

void clear_screen();
void wait_on_enter();
void Menu();
string Generate_password(int count);
int Data(string a, string b, string c);
void Print_all();
int Print(string name);
int callback(void *data, int argc, char **argv, char **azColName);
int Change_password(string _name, string new_data);
int Change_login(string _name, string new_data);
int Remove_data(string name);
