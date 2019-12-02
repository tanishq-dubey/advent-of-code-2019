#include <iostream>
#include <cmath>
#include <fstream>

using namespace std;

int fuel_calc(int mass) {
  return floor(mass/3.0) - 2;
}

int fuel_calc_repeated(int mass) {
  if (mass <= 0) {
    return 0;
  }

  int tmp = fuel_calc(mass);
  if (tmp <= 0) {
    return 0;
  }
  int tmp2 = fuel_calc_repeated(tmp);
  return tmp + tmp2;
}

int main() {
  ifstream infile("input.txt");

  int mass;
  int total = 0;
  int total_rep = 0;

  while(infile >> mass) {
    total = total + fuel_calc(mass);
    total_rep = total_rep + fuel_calc_repeated(mass);
  }

  cout << "Fuel needed: " << total << endl;
  cout << "Fuel needed accounting for fuel: " << total_rep << endl;
  cout << "For fun, the difference: " << total_rep - total << endl;
}
