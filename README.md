# kube-event-exporter
a container which exports kubernetes events to stdout

# Building

```bash
git clone https://github.com/JensWalter/kube-event-exporter.git
cd kube-event-exporter
docker build -t kube-event-exporter .
```

# Configuration

| variable | values | description |
|----------|---------|-------------|
|IGNORE_OLD_ENTRIES| TRUE, FALSE | default: TRUE, do not print entries older then 60 seconds |
| OUTPUT_FORMAT | PLAIN, JSON | default: PLAIN, print output as plain text |

# Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: kube-event-exporter
  labels:
    app: kube-event-exporter
spec:
  replicas: 1
  selector:
    matchLabels:
      app: kube-event-exporter
  template:
    metadata:
      labels:
        app: kube-event-exporter
    spec:
      serviceAccountName: kube-event-reader-account
      containers:
      - name: kube-event-exporter
        resources:
          requests:
            memory: "20Mi"
            cpu: "10m"
          limits:
            memory: "30Mi"
            cpu: "50m"
        image: {image-registry}/kube-event-exporter:latest
      imagePullSecrets:
      - name: cr-secret
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: kube-event-reader-account
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: kube-event-reader-role
rules:
- apiGroups: [""]
  resources: ["events"]
  verbs: ["get", "watch", "list"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: kube-event-reader-clusterrolebinding
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: kube-event-reader-role
subjects:
- kind: ServiceAccount
  name: kube-event-reader-account
```