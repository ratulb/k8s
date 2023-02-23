Kubernetes ingress default backend


To build and push to docker registry make sure $HOME/.docker/config.json exsists or edit accordi-ngly.

1) To run with istio, install istio and do a kubectl apply -f deployment.yaml/di.yaml 

Istio does not support k8s ingress default backend(https://github.com/istio/istio/issues/43566#issuecomment-1442207935).

2) To run with nginx ingress controller - run kubectl apply -f default-ingress.yaml

3) kubectl apply -f ic.yaml(Required only if ingressClassName: istio/nginx specified).  
 
