#include <stdio.h>
typedef unsigned long long int ulli;
/* MulSumLess
 * Nick Fagan
 * 2018 Dec 14
 * Finds the sum of all multiples of a and b less than n.
 */
ulli mulsumless(ulli a, ulli b, ulli n);

int main(int argc, char *argv) {
	ulli a,b,n;
	if(scanf("%llu %llu %llu",&a,&b,&n)!=3) return 1;
	ulli res = mulsumless(a,b,n);
	printf("%llu\n",res);
	return 0;
}

ulli mulsumless(ulli a, ulli b, ulli n) {
	ulli sum = 0;
	for(ulli i = 1; i < n; i++) {
		if(i%a==0 || i%b==0) sum+=i;
	}
	return sum;
}
