#!/bin/bash
kubectl delete -f application.yaml
sleep 2
kubectl apply -f application.yaml
