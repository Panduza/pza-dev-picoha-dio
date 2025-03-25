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
                "Checking the status of ${IMAGE_NAME}\n"\
                "---------------------------------"
        DATA=$(docker ps | grep ${IMAGE_NAME})
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
        img_ids=$(docker image ls | grep ${IMAGE_NAME})
        for img in $img_ids; do
            docker image rm -f $img
        done
        ;;

    *)
        echo "$0 [ARG]"
        echo -e "\t* stop   : stops the Docker instance"
        echo -e "\t* start  : start the Docker instance and/or connect you to bash"
        echo -e "\t* status : displays the status of your instance"
        echo -e "\t* build  : rebuilds the Docker image ${IMAGE_NAME}"
        echo -e "\t* run    : run the Docker instance"
        ;;
esac

cd -
