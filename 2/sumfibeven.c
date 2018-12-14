#include <stdio.h>
typedef unsigned long long int ulli;
/* SumFibEven
 * Nick Fagan
 * 2018 Dec 14
 * Sum of even fibonaccci numbers up to n
 */
ulli sfe(ulli max);

int main(int argc, char *argv) {
	ulli n;
	if(scanf("%llu",&n)!=1) return 1;
	ulli res = sfe(n);
	printf("%llu\n",res);
	return 0;
}


ulli sfe(ulli max) {
	//iterative fib
	//every third is even (starting at 2);
	//because: odd  + even = odd
	//         even + odd  = odd
	//         odd  + odd  = even.
	ulli prev = 1, curr = 2, next=0,sum=0;
	while(curr <= max) {
		sum += curr;
		//do a triple go to get next even one in curr
		for(int i = 0; i < 3; i++) {
			next = prev+curr;
			prev = curr;
			curr = next;
		}
	}
	return sum;
}
