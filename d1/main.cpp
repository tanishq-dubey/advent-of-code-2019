#include <iostream>
#include <cmath>
#include <fstream>

using namespace std;

int main() {
  string mass_file;
  cout << "Enter file: ";
  cin >> mass_file;

  ifstream infile(mass_file);

  int mass;
  int total = 0;
  while(infile >> mass) {
    total = total + floor(mass/3.0) - 2;
  }

  cout << "Fuel needed: " << total << endl;
}
