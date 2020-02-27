
#include <iostream>
#include <sstream>
#include <omp.h>
#include <mutex>
#include <cmath>
unsigned long long int digits_power_sum(unsigned long long int n, unsigned long long int power);

using namespace std;
int main(int argc, char** argv) {
	size_t to = 0;
	stringstream ss;
	ss << argv[1];
	ss >> to;
	unsigned long long int total = 0;
	mutex m;
	#pragma omp parallel for
	for(size_t i = 2; i < to; ++i) {
		if ( i == digits_power_sum(i, 5) ) {
			lock_guard<mutex> l(m);
			total += i;
			cout << total << endl;
		}
	}
}

unsigned long long int digits_power_sum(unsigned long long int n, unsigned long long int power) {
	unsigned long long int res = 0;
	while(n > 0) {
		res += pow((n % 10), power);
		n /= 10;
	}
	return res;
}
