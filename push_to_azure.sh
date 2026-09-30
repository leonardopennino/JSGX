#!/bin/zsh
function pta () {
  if (( "$#" == 1 )); then
	echo "Pushing $1 to server"
	scp $1 azurevm:/home/azureuser/JSGX
	return
  fi
  make RELEASE=1 tee-client tee-server jar
  scp ./tee-client azurevm:/home/azureuser/JSGX/
  scp ./tee-server azurevm:/home/azureuser/JSGX/
  scp ./java-tee.jar azurevm:/home/azureuser/JSGX/
  echo "All files pushed to server"
}

alias mm="make clean && make SGX=1"
alias mmd="make clean && make SGX=1 DEBUG=1"
alias mmr="make clean && make all RELEASE=1"
alias gr="gramine-direct rust"
alias grx="gramine-sgx rust"
