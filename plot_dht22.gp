# Gunakan terminal ringan (wxt lebih stabil di Windows)
set terminal wxt size 1600,800 enhanced font 'Arial,10'
set output

# Format file CSV
set datafile separator ";"

# Judul dan label
set title "Data Sensor DHT22 Selama 10 Hari"
set xlabel "Waktu Pengukuran"
set ylabel "Nilai Sensor"
set grid
set key outside right top box

# Gaya garis
set style line 1 lc rgb "#1f77b4" lw 2
set style line 2 lc rgb "#ff7f0e" lw 2

# Format waktu di sumbu X
set xdata time
set timefmt "%d/%m/%Y %H:%M"
set format x "%d/%m\n%H:%M"
set xtics rotate by -30

# Batas tampilan (opsional, bisa disesuaikan)
#set xrange ["30/09/2025 00:00":"09/10/2025 23:59"]

# Plot dengan sampling (ambil tiap 100 titik biar ringan)
plot "Hasil data DHT 22 10 Hari_fix.csv" every 100 using 1:2 with lines ls 1 title "Kelembapan (%)", \
     "Hasil data DHT 22 10 Hari_fix.csv" every 100 using 1:3 with lines ls 2 title "Suhu (°C)"

pause -1 "Tekan Enter untuk menutup grafik..."
