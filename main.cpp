#include "Header.h"

// menu
void Menu() {
  cout << "Choose the action:" << endl
       << menu_one << endl
       << menu_two << endl
       << menu_tree << endl
       << menu_four << endl
       << menu_five << endl
       << menu_six << endl
       << "(0) Exit" << endl;

  cin >> menu_act;
}

int main() {
  srand(time(NULL));

  cout << warning << endl;
  cout << "If you see a problem with program, notify a developer!" << endl;

  Menu();

  // main cycle
  while (menu_act != 0) {
    // creating password
    if (menu_act == 1) {
      clear_screen();

      cout << menu_one << endl;
      cout << warning << endl;
      cout << "Enter count of symbols:" << endl;
      cin >> symbols_count;

      // Generation of password
      Generate_password(symbols_count);

      cout << "Do you want to save the password?" << endl
           << "(1) Yes" << endl
           << "(2) No" << endl;
      cin >> act;
      if (act == 1) {
        cout << "Dont forget to copy your password in to the clipboard!!!"
             << endl;

        // Pause with clear
        wait_on_enter();
        clear_screen();

        // Entering a data
        cout << "Please, check the spelling of the data is correct!" << endl
             << "Enter service name:" << endl;
        cin >> name;
        cout << "Enter login (email, phone number, etc):" << endl;
        cin >> login;
        cout << "Enter password:" << endl;
        cin >> password;
        // saving password
        Data(name, login, password);
      }

      // Pause
      wait_on_enter();
      // Clear console
      clear_screen();
      Menu();
    }
    // saving password
    else if (menu_act == 2) {
      // Clear console
      clear_screen();

      cout << menu_two << endl;
      // entering a data
      cout << warning << endl;
      cout << "Please, check the spelling of the data is correct!" << endl
           << "Enter service name:" << endl;
      cin >> name;
      cout << "Enter login (email, phone number, etc):" << endl;
      cin >> login;
      cout << "Enter password:" << endl;
      cin >> password;

      Data(name, login, password);

      // Pause with clear console
      wait_on_enter();
      clear_screen();

      Menu();
    }
    // show a password
    else if (menu_act == 3) {
      // Clear console
      clear_screen();

      cout << menu_tree << endl;
      ;
      cout << opa << endl;
      cin >> act;

      if (act == 1) {

        Print_all();
      } else if (act == 2) {
        cout << "Enter the name of service" << endl;
        cin >> name;
        Print(name);
      } else {
        cout << "Error! Incorrect value!" << endl;
        cout << opa << endl;
      }

      // Pause with clear console
      wait_on_enter();
      clear_screen();

      Menu();
    }

    // chagne a password
    else if (menu_act == 4) {
      // Clear console
      clear_screen();

      cout << menu_four << endl;
      cout << warning << endl;
      cout << "Enter the name of service" << endl;
      cin >> _name;
      cout << "Enter a new password" << endl;
      cin >> new_data;

      Change_password(_name, new_data);
      Print_all();

      // Pause with clear console
      wait_on_enter();
      clear_screen();

      Menu();
    }

    // change a login
    else if (menu_act == 5) {
      // CLear console
      clear_screen();

      cout << menu_five << endl;
      cout << warning << endl;
      cout << "Enter a service name:" << endl;
      cin >> _name;
      cout << "Enter a new login: " << endl;
      cin >> new_data;

      Change_login(_name, new_data);
      Print_all();

      // Pause with clear console
      wait_on_enter();
      clear_screen();

      Menu();
    }

    // Remove a password
    else if (menu_act == 6) {
      // Clear console
      clear_screen();

      cout << menu_six << endl;
      cout << warning << endl;
      cout << "Enter the name of service, which you want to remove the password"
           << endl;
      cin >> name;

      Remove_data(name);
      Print_all();

      // Pause with clear console
      wait_on_enter();
      clear_screen();

      Menu();
    }

    // wrong action
    else {
      // Clear console
      clear_screen();

      cout << "Wrong action!" << endl;

      // Pause with clear console
      wait_on_enter();
      clear_screen();

      Menu();
    }
  }
  return 0;
}
