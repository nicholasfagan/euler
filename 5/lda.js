/*setup console interface*/
var readline = require('readline');
var rl = readline.createInterface({
	input:process.stdin,
	output:process.stdout
});


/*factor methods*/
const newpfact = (a,b)=>cb=>cb(a,b);
//get stuff
const getFact = (a,b)=>a;
const getPow = (a,b)=>b;
//compare
const cmpFact = (a,b)=>a(getFact) - b(getFact);
const cmpPow = (a,b)=>a(getPow) - b(getPow);
// base ^ pow
const evalf = f=>Math.pow(f(getFact),f(getPow));
/* get all prime factors*/
const pfacts = n=>{
	if(isNaN(n) || n<=1) return [];
	var res = [];
	var d = 2;
	while(n > 1) {
		if(n%d==0) {
			var p = 0;
			while(n%d==0) {
				n /= d;
				p++;
			}
			res.push(newpfact(d,p));
		}
		d++;
	}
	return res;
};
//merge 2 lists of factors, 
//getting only the largest power for each base.
const fmerge = (l1,l2) => {
	if(l1.length == 0) return l2;
	if(l2.length == 0) return l1;

	l1 = l1.sort(cmpFact);
	l2 = l2.sort(cmpFact);
	
	var res = [];
	var i1 = 0;
	var i2 = 0;
	
	while(i1 < l1.length && i2 < l2.length) {
		var fcmp = cmpFact(l1[i1],l2[i2]);
		if( fcmp < 0) {
			//l2 > l1
			res.push(l1[i1++]);
		}else if(fcmp > 0) {
			//l2 < l1
			res.push(l2[i2++]);
		}else {
			//factors are equal, pick biggest one.
			var pcmp = cmpPow(l1[i1],l2[i2]);
			if(pcmp <= 0) {
				//pow of 1 < 2
				// pick 2
				res.push(l2[i2]);
			}else {
				// pick 1
				res.push(l1[i1]);
			}
			i1++;
			i2++;
		}
	}
	//now add rest if one list finished before other.	
	while(i1<l1.length) {
		res.push(l1[i1++]);
	}
	while(i2<l2.length) {
		res.push(l2[i2++]);
	}
	return res;
}

const mul = (a,b) => {
	return a*b;
};

//return NaN or the lowest number that can be divided evenly by every number 1 to n
const lda = n=> isNaN(n) || n <= 1 ? NaN : Array.from({length:n},(a,i)=>i+1).map(pfacts).reduce(fmerge).map(evalf).reduce(mul);
/* console interaction*/
rl.on('line',l=>console.log(lda(parseInt(l))));
