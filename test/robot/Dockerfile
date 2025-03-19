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
RUN apt install -y python3 python3-pip python3-venv
RUN python3 -m venv venv && . venv/bin/activate
RUN pip install -r requirements.txt

# Stage 2: Set up Nginx
# Install Nginx
RUN apt install -y nginx systemctl
# Copy your custom Nginx configuration file and content
COPY ./www/nginx.conf /etc/nginx/conf.d/default.conf

# Set the command to start Nginx
CMD ["nginx", "-g", "daemon off;"]
