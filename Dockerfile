FROM python:3.12-slim

ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /app

# Qt/PySide runtime deps (inside container)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libglib2.0-0 libdbus-1-3 \
    libegl1 libgl1 \
    libfontconfig1 libfreetype6 \
    libxkbcommon0 libxkbcommon-x11-0 \
    libxcb1 libxcb-cursor0 libxcb-icccm4 libxcb-image0 libxcb-keysyms1 \
    libxcb-randr0 libxcb-render-util0 libxcb-shape0 libxcb-shm0 libxcb-sync1 \
    libxcb-xfixes0 libxcb-xinerama0 \
    libx11-6 libxext6 libxrender1 libxi6 libxrandr2 libxtst6 \
  && rm -rf /var/lib/apt/lists/*

# Install Python deps (fail build if PySide6 missing)
COPY requirements.txt /app/requirements.txt
RUN python3 -m pip install --upgrade pip \
 && python3 -m pip install --no-cache-dir -r /app/requirements.txt \
 && python3 -m pip show PySide6

# Copy app source
COPY . /app

CMD ["python3", "main.py"]
