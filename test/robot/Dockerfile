# Use the official Python image as the base image
FROM ubuntu:24.04 

# Set the working directory
WORKDIR /home

# Copy the current directory contents into the container at /home
COPY . .

# Update Ubuntu Software repository
RUN apt update && apt upgrade -y

# Stage 1: Build the Python application
# Install Python and pip
RUN apt install -y python3 python3-pip 

# Install requirements python packet
# No need to create virtual environment here: break system packages
RUN pip install --break-system-packages -r requirements.txt

# Stage 2: Set up Nginx
# Install Nginx
RUN apt install -y nginx

# I also use python dependence on app.py
RUN pip install --break-system-packages Flask

# Copy your custom Nginx configuration file
COPY ./panduza-server.conf /etc/nginx/sites-enabled/

# Set the command to start Nginx
CMD ["nginx", "-g", "daemon off;"]
