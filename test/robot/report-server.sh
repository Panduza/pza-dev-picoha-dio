#!/bin/bash
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
                "Connect USB: ${pico_port}\n"\
                "--------------------------------------------------"
        docker run -it --rm -d --name ${container_name} \
            --device=${pico_port} \
            -p 8080:80 \
            ${IMAGE_NAME}
        ;;

    run)
        echo -e " --------------------------------------------------\n"\
                "Run instance of img: ${IMAGE_NAME}.\n"\
                "Connect USB: ${pico_port}\n"\
                "--------------------------------------------------"
        docker run -it --entrypoint bash \
            --device=${pico_port} \
            ${IMAGE_NAME}
        ;;

    clean)
        echo -e " --------------------------------------------------\n"\
                "Remove image ${IMAGE_NAME}.\n"\
                "--------------------------------------------------"
        # Get the image IDs for the specified image name
        img_ids=$(docker image ls --format "{{.Repository}}:{{.Tag}}->{{.ID}}" | grep -E "${IMAGE_NAME}|<none>")
        # Ask for confirmation before deleting the images
        echo "Are you sure you want to delete the following images ? (y/n):"
        for img in ${img_ids}; do
            echo -e $img
        done
        read answer
        if [ "$answer" = "y" ]; then
            # Loop through each image ID and remove the image
            for img in $(echo -e $img_ids | cut -d '>' -f 2); do
                docker image rm -f $img && echo "Image deleted successfully." || echo "Failed to Delete Image."
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
        echo -e "\t* clean  : Delete docker image ${IMAGE_NAME} and image called 'None'"
        ;;
esac

cd -
