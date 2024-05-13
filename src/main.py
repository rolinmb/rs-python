import matplotlib.pyplot as plt
if __name__ == "__main__":
	data = [1.0, 2.0, 3.0, 4.0, 5.0]
	print("\n(main.py) data: Vec<f64> from src/main.rs =")
	print(data)
	plt.plot(data)
	plt.xlabel("Index")
	plt.ylabel("Value")
	plt.ylabel("Simple Plot")
	plt.show()