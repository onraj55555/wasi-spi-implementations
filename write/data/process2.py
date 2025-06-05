import pandas as pd
import matplotlib
matplotlib.use('Qt5Agg')  # Enable GUI plot window
import matplotlib.pyplot as plt
import numpy as np


# Read the CSV (no headers)
df = pd.read_csv("results/wasm-writer-64-speedtest.csv", header=None)

# Extract the OK/ERR column
status = df[0].tolist()

# ---- Cumulative Error Rate Calculation ----
cumulative_error_rate = []
err_count = 0

for i, val in enumerate(status):
    if val == "ERR":
        err_count += 1
    cumulative_error_rate.append(err_count / (i + 1))

# ---- Plotting ----
plt.figure(figsize=(12, 6))
plt.plot(cumulative_error_rate, color='red')
plt.title("Cumulative Error Rate WASM Implementation 1 Word")
plt.ylabel("Cumulative ERR Rate")

# Set x-axis labels to range from 10 to 10000 without changing the data
num_points = len(cumulative_error_rate)
desired_labels = np.linspace(10, 10000, num=2)  # Change `num` for more/fewer ticks
tick_positions = np.linspace(0, num_points - 1, num=2)
plt.xticks(tick_positions, [f"{int(x)}" for x in desired_labels])

plt.xlabel("Frequency (kHz)")
plt.grid(True)
plt.tight_layout()
plt.savefig("wasm-1-word.png")