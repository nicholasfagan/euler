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
/* get all prime factors*/
const pfacts = n=>{
	if(isNaN(n) || n<=1) return [];
	var res = [];
	var d = 2;
	while(n > 1) {
		//find and count them
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

const countFact = n => pfacts(n).length;

//count distinct prime numbers
//returns first number to have
// n distinct prime factors
const cdpf = n => {
	var i = 1;
	while(true) {
		i++;
		var pass = true;
		for(var j = 0; j < n; j++) {
			var fc = countFact(i+j);
			if(fc != n) {
				pass = false;
				break;
			}	
		}
		if(pass) break;
	}
	return i;
};



rl.on('SIGINT',()=>process.exit(0));
rl.on('line',l=>console.log(cdpf(parseInt(l))));
