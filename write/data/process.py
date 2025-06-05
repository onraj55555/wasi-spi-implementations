import csv
import matplotlib.pyplot as plt
import sys

# --- Load error data ---
data_file = sys.argv[1]
speeds_file = sys.argv[2]
name = sys.argv[3]

# --- Step 1: Read speed values ---
speed_values = []

with open(speeds_file, 'r', encoding='utf-8') as f:
    reader = csv.reader(f)
    for row in reader:
        if not row:
            continue
        line = row[0]
        if line.startswith("Speed:"):
            try:
                value = int(line.split(":")[1].strip().replace("kHz", "").strip())
                speed_values.append(value)
            except:
                continue

# --- Step 2: Read quality data and compute cumulative error rate ---
error_rate_over_time = []
good_count = 0
bad_count = 0

with open(data_file, 'r', encoding='utf-8') as f:
    reader = csv.reader(f)
    for row in reader:
        if not row or len(row) < 1:
            continue
        first_col = row[0].strip()
        if first_col == 'OK':
            good_count += 1
        else:
            bad_count += 1
        total = good_count + bad_count
        error_rate = bad_count / total
        error_rate_over_time.append(error_rate)

# --- Step 3: Match length of data and speeds ---
if len(speed_values) != len(error_rate_over_time):
    1/0
min_len = min(len(speed_values), len(error_rate_over_time))
speeds = speed_values[:min_len]
errors = error_rate_over_time[:min_len]

# --- Step 4: Plot ---
plt.figure(figsize=(12, 6))
plt.plot(speeds, errors, marker='o', linewidth=0.5, label='Cumulative Error Rate')

plt.xlabel('Speed (kHz)')
plt.ylabel('Cumulative Error Rate')
plt.title('Cumulative Error Rate vs Speed')
plt.grid(True)
plt.legend()
plt.tight_layout()

# Save plot
plt.savefig(name)
print("Plot saved")
