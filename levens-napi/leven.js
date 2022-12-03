function leven(str1,str2) {
	const matrix = Array.from({length: str2.length + 1}, (v,idx) => Array.from({length: str1.length + 1}, (v2,idx2) => idx === 0 ? idx2 : idx ));

	for (let i = 0; i < str2.length; i++ ) {
		for (let j = 0; j < str1.length; j++) {
			const min = Math.min(...[+matrix[i][j],+matrix[i][j + 1],+matrix[i + 1][j]]);
			if (str1[i] !== str2[j]) {
				matrix[i + 1][j + 1] = min + 1;
			} else {
				matrix[i + 1][j + 1] = min;
			}
		}
	}
	return matrix[str2.length][str1.length];
}

module.exports = {
	leven
};
