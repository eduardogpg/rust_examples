fn quick_sort(vector: Vec<i32>) -> Vec<i32> {
	if vector.len() > 1{
		let mut less = Vec::new();
		let mut equals = Vec::new();
		let mut greater = Vec::new();

		let pivote = vector[0];

		for value in vector{

			if value > pivote{
				greater.push(value)
			}
			if value < pivote{
				less.push(value)
			}
			if value == pivote{
				equals.push(value)
			}
		}

		let mut final_vector = Vec::new();
		final_vector.extend(quick_sort(less));
		final_vector.extend(equals);
		final_vector.extend(quick_sort(greater));

		return final_vector;

	}else{
		return vector
	}
}


fn main() {
	let vector = vec![9,7,18,29,5,3,99,77,8,1,19,199,13,15,1];
	let result = quick_sort(vector);
	println!("{:?}", result );
}
