#!/bin/bash
ACTION=$1
IMG='robot-report-server'

cd $(dirname "$0")

case $ACTION in
    build)
        echo -e " -------------------------------------------\n"\
                "Generation docker image: ${IMG}\n"\
                "-------------------------------------------"
        docker build . -t ${IMG}
        ;;
    status)
        echo -e " ---------------------------------\n"\
                "Checking the status of ${IMG}\n"\
                "---------------------------------"
        DATA=$(docker ps | grep ${IMG})
        [[ -z $DATA ]] && echo "No Process" || echo "$DATA"
        ;;

    stop)
        echo -e " -----------------------------------------\n"\
                "Stopping docker instance ${IMG}\n"\
                "-----------------------------------------"
        docker stop ${IMG}
        ;;

    start)
        echo -e " --------------------------------------------------\n"\
                "Starting and connecting the instance ${IMG}\n"\
                "--------------------------------------------------"
        docker run -it --rm -d --name ${IMG} -p 8080:80 nginx
        ;;

    run)
        echo -e " --------------------------------------------------\n"\
                "Run instance ${IMG}.\n"\
                "--------------------------------------------------"
        docker run -it --entrypoint sh ${IMG}
        ;;
    *)
        echo "$0 [stop/start/status/build]"
        echo -e "\t* stop   : stops the Docker instance"
        echo -e "\t* start  : start the Docker instance and/or connect you to bash"
        echo -e "\t* status : displays the status of your instance"
        echo -e "\t* build  : rebuilds the Docker image ${IMG}"
        echo -e "\t* run    : run the Docker instance"
        exit 0
        ;;
esac

cd -
