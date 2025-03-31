#!/bin/bash
#
# This Bash script allows you to manage a Docker instance for a Robot Framework report server with Nginx. 
# It supports various actions such as building the Docker image, starting, stopping, running, and cleaning up Docker images.
#

ACTION=$1
IMAGE_NAME=img_nginx_robot

container_name=robot-report-server
pico_port='/dev/ttyACM0'

cd $(dirname "$0")

case $ACTION in
    build)
        echo -e " -------------------------------------------\n"\
                "Generation docker image: ${IMAGE_NAME}\n"\
                "-------------------------------------------"
        docker build . -t ${IMAGE_NAME}
        ;;
    status)
        echo -e " ---------------------------------\n"\
                "Get the status of ${IMAGE_NAME}\n"\
                "---------------------------------"
        # Get the status of the specified image name
        DATA=$( $PS_DATA | grep ${IMAGE_NAME})
        [[ -z $DATA ]] && echo "No Process" || echo "$DATA"
        ;;

    stop)
        echo -e " -----------------------------------------\n"\
                "Stopping docker instance ${container_name}\n"\
                "-----------------------------------------"
        docker stop ${container_name}
        ;;

    start)
        echo -e " --------------------------------------------------\n"\
                "Starting and connecting the instance ${container_name}\n"\
                "Container base on ${IMAGE_NAME}"\
                "Connect USB: ${pico_port}\n"\
                "Report use port: 8080\n"\
                "--------------------------------------------------"
        docker run -it --rm -d --name ${container_name} \
            --device=${pico_port} \
            -p 8080:80 \
            ${IMAGE_NAME}
        ;;

    run)
        echo -e " --------------------------------------------------\n"\
                "Enter in container: ${container_name}.\n"\
                "--------------------------------------------------"
        docker exec -it ${container_name} bash
        ;;

    clean)
        echo -e " --------------------------------------------------\n"\
                "Remove image ${IMAGE_NAME}.\n"\
                "--------------------------------------------------"
        # Get the image IDs for the specified image name
        images_infos=$(docker image ls --format "{{.Repository}}:{{.Tag}}:{{.ID}}" | grep -E "${IMAGE_NAME}|<none>")

        # Ask for confirmation before deleting the images
        for images in ${images_infos}; do
            echo -e $images
        done        
        echo "Are you sure you want to delete the following images ? (y/n):"
        read answer

        if [ "$answer" = "y" ]; then
            # Loop through each image ID and remove the image
            for img in $images_infos; do #
                img_id=$(echo $img | cut -d ':' -f 3)
                docker image rm -f $img_id && echo "Image deleted successfully." || echo "Failed to Delete Image."
            done
        else
            echo "Operation cancelled."
        fi
        ;;

    *)
        echo "$0 [ARG]"
        echo -e "\t* run    : Run the Docker instance"
        echo -e "\t* stop   : Stops the Docker instance"
        echo -e "\t* start  : Start the Docker instance and/or connect you to bash"
        echo -e "\t* status : Get the status of your instance"
        echo -e "\t* build  : Builds the Docker image ${IMAGE_NAME}"
        echo -e "\t* clean  : Delete docker image ${IMAGE_NAME} and image called <none>"
        ;;
esac

cd -
