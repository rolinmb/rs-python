import matplotlib.pyplot as plt
if __name__ == "__main__":
	floats = [1.0, 2.0, 3.0, 4.0, 5.0]
	print("\n(main.py) floats: Vec<f64> from src/main.rs =")
	print(floats)
	fig = plt.plot(floats)
	plt.xlabel("Index")
	plt.ylabel("Value")
	plt.ylabel("Simple Plot")
	plt.show()