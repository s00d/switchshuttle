import type { Command } from '../types';

export interface Template {
  id: string;
  name: string;
  description: string;
  category: string;
  icon: string;
  commands: Command[];
  tags: string[];
}

export const templates: Template[] = [
  {
    id: 'development-category',
    name: 'Development',
    description: 'General software development tools and workflows',
    category: 'Development',
    icon: '💻',
    tags: ['development', 'programming', 'coding'],
      commands: [
      { id: 'dev-setup-project', name: '🚀 Init Project npm', command: 'mkdir [project_name] && cd [project_name] && npm init -y', inputs: { project_name: 'my-app' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'dev-install-deps', name: '📦 Install Dependencies npm', command: 'npm install [package_name]', inputs: { package_name: 'lodash' }, hotkey: 'Ctrl+Shift+I' },
      { id: 'dev-run-script', name: '▶️ Run Script npm', command: 'npm run [script_name]', inputs: { script_name: 'run' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'dev-build', name: '📦 Build Project npm', command: 'npm run build', hotkey: 'Ctrl+Shift+B' },
      { id: 'dev-test', name: '🧪 Run Tests npm', command: 'npm test', hotkey: 'Ctrl+Shift+T' },
      { id: 'dev-lint', name: '🔍 Lint Code npm', command: 'npm run lint', hotkey: 'Ctrl+Shift+L' },
      { id: 'dev-format', name: '✨ Format Code npm', command: 'npm run format', hotkey: 'Ctrl+Shift+F' },
      { id: 'dev-watch', name: '👁️ Watch Mode npm', command: 'npm run dev', hotkey: 'Ctrl+Shift+W' },
      { id: 'dev-debug', name: '🐛 Debug Mode npm', command: 'npm run debug', hotkey: 'Ctrl+Shift+D' },
      { id: 'dev-clean', name: '🧹 Clean Install npm', command: 'rm -rf node_modules package-lock.json && npm install', hotkey: 'Ctrl+Shift+C' },
      { id: 'dev-update', name: '🔄 Update Dependencies npm', command: 'npm update', hotkey: 'Ctrl+Shift+U' },
      { id: 'dev-audit', name: '🔍 Audit Security npm', command: 'npm audit', hotkey: 'Ctrl+Shift+A' },
      { id: 'dev-outdated', name: '📋 Check Outdated npm', command: 'npm outdated', hotkey: 'Ctrl+Shift+O' },
      { id: 'dev-publish', name: '📤 Publish Package npm', command: 'npm publish', hotkey: 'Ctrl+Shift+P' },
      { id: 'dev-version', name: '🏷️ Bump Version npm', command: 'npm version [version_type]', inputs: { version_type: 'patch' }, hotkey: 'Ctrl+Shift+V' },
      { id: 'dev-git-init', name: '📚 Init Repository git', command: 'git init && git add . && git commit -m "Initial commit"', hotkey: 'Ctrl+Shift+G' },
      { id: 'dev-readme', name: '📖 Create README file', command: 'touch README.md', hotkey: 'Ctrl+Shift+R' },
      { id: 'dev-license', name: '📄 Create LICENSE file', command: 'touch LICENSE', hotkey: 'Ctrl+Shift+L' },
      { id: 'dev-gitignore', name: '🚫 Create .gitignore file', command: 'touch .gitignore', hotkey: 'Ctrl+Shift+I' },
      { id: 'dev-env', name: '🌍 Create .env files', command: 'touch .env.example && touch .env', hotkey: 'Ctrl+Shift+E' }
    ]
  },
  {
    id: 'devops-category',
    name: 'DevOps',
    description: 'DevOps tools and practices for continuous integration and deployment',
    category: 'DevOps',
    icon: '🔧',
    tags: ['devops', 'ci-cd', 'automation', 'deployment'],
      commands: [
      { id: 'devops-docker-build', name: '🐳 Build Image Docker', command: 'docker build -t [image_name] .', inputs: { image_name: 'my-image' }, hotkey: 'Ctrl+Shift+B' },
      { id: 'devops-docker-run', name: '▶️ Run Container Docker', command: 'docker run -d --name [container_name] -p [port]:[port] [image_name]', inputs: { container_name: 'my-container', port: '3000', image_name: 'my-image' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'devops-docker-compose', name: '🚀 Compose Up Docker', command: 'docker-compose up -d', hotkey: 'Ctrl+Shift+U' },
      { id: 'devops-docker-stop', name: '⏹️ Compose Down Docker', command: 'docker-compose down', hotkey: 'Ctrl+Shift+D' },
      { id: 'devops-k8s-pods', name: '📦 Get Pods Kubernetes', command: 'kubectl get pods -A', hotkey: 'Ctrl+Shift+P' },
      { id: 'devops-k8s-services', name: '🌐 Get Services Kubernetes', command: 'kubectl get services -A', hotkey: 'Ctrl+Shift+S' },
      { id: 'devops-k8s-apply', name: '📋 Apply Manifest Kubernetes', command: 'kubectl apply -f [manifest.yaml]', inputs: { manifest: 'manifest.yaml' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'devops-k8s-logs', name: '📋 View Logs Kubernetes', command: 'kubectl logs [pod_name] -f', inputs: { pod_name: 'my-pod' }, hotkey: 'Ctrl+Shift+L' },
      { id: 'devops-k8s-exec', name: '🔧 Exec Pod Kubernetes', command: 'kubectl exec -it [pod_name] -- /bin/bash', inputs: { pod_name: 'my-pod' }, hotkey: 'Ctrl+Shift+E' },
      { id: 'devops-terraform-init', name: '🏗️ Init Terraform', command: 'terraform init', hotkey: 'Ctrl+Shift+I' },
      { id: 'devops-terraform-plan', name: '📋 Plan Terraform', command: 'terraform plan', hotkey: 'Ctrl+Shift+P' },
      { id: 'devops-terraform-apply', name: '🚀 Apply Terraform', command: 'terraform apply', hotkey: 'Ctrl+Shift+A' },
      { id: 'devops-terraform-destroy', name: '🗑️ Destroy Terraform', command: 'terraform destroy', hotkey: 'Ctrl+Shift+D' },
      { id: 'devops-ansible-playbook', name: '📜 Run Playbook Ansible', command: 'ansible-playbook [playbook.yml]', inputs: { playbook: 'playbook.yml' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'devops-jenkins-build', name: '🔨 Trigger Build Jenkins', command: 'curl -X POST [jenkins_url]/job/[job_name]/build', inputs: { jenkins_url: 'http://localhost:8080', job_name: 'build' }, hotkey: 'Ctrl+Shift+J' },
      { id: 'devops-gitlab-ci', name: '🔄 Run Pipeline GitLab CI', command: 'gitlab-ci-local', hotkey: 'Ctrl+Shift+G' },
      { id: 'devops-github-actions', name: '⚡ Run Workflow GitHub Actions', command: 'gh workflow run [workflow_name]', inputs: { workflow_name: 'build' }, hotkey: 'Ctrl+Shift+H' },
      { id: 'devops-monitoring', name: '📊 Check Metrics System', command: 'htop', hotkey: 'Ctrl+Shift+M' },
      { id: 'devops-logs', name: '📋 View Logs Application', command: 'tail -f [log_file]', inputs: { log_file: 'app.log' }, hotkey: 'Ctrl+Shift+L' }
    ]
  },
  {
    id: 'frontend-category',
    name: 'Frontend',
    description: 'Frontend development frameworks and tools',
    category: 'Frontend',
    icon: '🎨',
    tags: ['frontend', 'ui', 'javascript', 'css', 'html'],
      commands: [
      { id: 'frontend-react-create', name: '⚛️ Create App React', command: 'npx create-react-app [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'frontend-vue-create', name: '💚 Create App Vue', command: 'npm create vue@latest [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+V' },
      { id: 'frontend-angular-create', name: '🅰️ Create App Angular', command: 'ng new [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'frontend-svelte-create', name: '⚡ Create App Svelte', command: 'npm create svelte@latest [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+S' },
      { id: 'frontend-nextjs-create', name: '⚡ Create App Next.js', command: 'npx create-next-app@latest [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'frontend-dev-server', name: '▶️ Start Dev Server npm', command: 'npm run dev', hotkey: 'Ctrl+Shift+D' },
      { id: 'frontend-build', name: '📦 Build Project npm', command: 'npm run build', hotkey: 'Ctrl+Shift+B' },
      { id: 'frontend-preview', name: '👁️ Preview Build npm', command: 'npm run preview', hotkey: 'Ctrl+Shift+P' },
      { id: 'frontend-test', name: '🧪 Run Tests npm', command: 'npm test', hotkey: 'Ctrl+Shift+T' },
      { id: 'frontend-lint', name: '🔍 Lint Code npm', command: 'npm run lint', hotkey: 'Ctrl+Shift+L' },
      { id: 'frontend-format', name: '✨ Format Code npm', command: 'npm run format', hotkey: 'Ctrl+Shift+F' },
      { id: 'frontend-component', name: '🧩 Create Component file', command: 'touch src/components/[component_name].jsx', inputs: { component_name: 'MyComponent' }, hotkey: 'Ctrl+Shift+C' },
      { id: 'frontend-page', name: '📄 Create Page file', command: 'touch src/pages/[page_name].jsx', inputs: { page_name: 'MyPage' }, hotkey: 'Ctrl+Shift+P' },
      { id: 'frontend-style', name: '🎨 Create Style file', command: 'touch src/styles/[style_name].css', inputs: { style_name: 'main' }, hotkey: 'Ctrl+Shift+Y' },
      { id: 'frontend-asset', name: '📁 Create Asset folder', command: 'mkdir -p public/[asset_type]', inputs: { asset_type: 'images' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'frontend-router', name: '🛣️ Install Router React', command: 'npm install react-router-dom', hotkey: 'Ctrl+Shift+R' },
      { id: 'frontend-state', name: '📊 Install State Management Redux', command: 'npm install @reduxjs/toolkit react-redux', hotkey: 'Ctrl+Shift+S' },
      { id: 'frontend-ui', name: '🎨 Install UI Library MUI', command: 'npm install @mui/material @emotion/react @emotion/styled', hotkey: 'Ctrl+Shift+U' },
      { id: 'frontend-storybook', name: '📚 Init Storybook', command: 'npx storybook@latest init', hotkey: 'Ctrl+Shift+K' },
      { id: 'frontend-pwa', name: '📱 Install PWA Workbox', command: 'npm install workbox-webpack-plugin', hotkey: 'Ctrl+Shift+P' }
    ]
  },
  {
    id: 'backend-category',
    name: 'Backend',
    description: 'Backend development frameworks and server-side tools',
    category: 'Backend',
    icon: '⚙️',
    tags: ['backend', 'api', 'server', 'database'],
      commands: [
      { id: 'backend-node-init', name: '🟢 Init Project Node.js', command: 'npm init -y', hotkey: 'Ctrl+Shift+N' },
      { id: 'backend-express-create', name: '🚀 Install Express App', command: 'npm install express cors helmet morgan', hotkey: 'Ctrl+Shift+E' },
      { id: 'backend-fastify-create', name: '⚡ Install Fastify App', command: 'npm install fastify', hotkey: 'Ctrl+Shift+F' },
      { id: 'backend-koa-create', name: '🌊 Install Koa App', command: 'npm install koa @koa/router', hotkey: 'Ctrl+Shift+K' },
      { id: 'backend-nest-create', name: '🪺 Create App NestJS', command: 'npm install -g @nestjs/cli && nest new [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'backend-django-create', name: '🐍 Create Project Django', command: 'django-admin startproject [project_name]', inputs: { project_name: 'my-app' }, hotkey: 'Ctrl+Shift+D' },
      { id: 'backend-flask-create', name: '🍶 Create App Flask', command: 'mkdir [app_name] && cd [app_name] && touch app.py', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+F' },
      { id: 'backend-fastapi-create', name: '⚡ Install FastAPI App', command: 'pip install fastapi uvicorn', hotkey: 'Ctrl+Shift+A' },
      { id: 'backend-spring-create', name: '🍃 Create App Spring Boot', command: 'curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o [app_name].zip', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+S' },
      { id: 'backend-laravel-create', name: '🦋 Create App Laravel', command: 'composer create-project laravel/laravel [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+L' },
      { id: 'backend-rails-create', name: '💎 Create App Rails', command: 'rails new [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'backend-go-create', name: '🐹 Create App Go', command: 'mkdir [app_name] && cd [app_name] && go mod init [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+G' },
      { id: 'backend-rust-create', name: '🦀 Create App Rust', command: 'cargo new [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'backend-server-start', name: '▶️ Start Server npm', command: 'npm start', hotkey: 'Ctrl+Shift+S' },
      { id: 'backend-server-dev', name: '🔄 Start Dev Server npm', command: 'npm run dev', hotkey: 'Ctrl+Shift+D' },
      { id: 'backend-test', name: '🧪 Run Tests npm', command: 'npm test', hotkey: 'Ctrl+Shift+T' },
      { id: 'backend-lint', name: '🔍 Lint Code npm', command: 'npm run lint', hotkey: 'Ctrl+Shift+L' },
      { id: 'backend-build', name: '📦 Build Project npm', command: 'npm run build', hotkey: 'Ctrl+Shift+B' },
      { id: 'backend-migrate', name: '🗄️ Run Migrations npm', command: 'npm run migrate', hotkey: 'Ctrl+Shift+M' },
      { id: 'backend-seed', name: '🌱 Seed Database npm', command: 'npm run seed', hotkey: 'Ctrl+Shift+S' }
    ]
  },
  {
    id: 'database-category',
    name: 'Database',
    description: 'Database management and operations',
    category: 'Database',
    icon: '🗄️',
    tags: ['database', 'sql', 'nosql', 'orm'],
      commands: [
      { id: 'db-postgres-start', name: '🐘 Start Service PostgreSQL', command: 'brew services start postgresql', hotkey: 'Ctrl+Shift+P' },
      { id: 'db-postgres-stop', name: '⏹️ Stop Service PostgreSQL', command: 'brew services stop postgresql', hotkey: 'Ctrl+Shift+S' },
      { id: 'db-postgres-connect', name: '🔗 Connect Database PostgreSQL', command: 'psql -U [username] -d [database]', inputs: { username: 'user', database: 'testdb' }, hotkey: 'Ctrl+Shift+C' },
      { id: 'db-mysql-start', name: '🐬 Start Service MySQL', command: 'brew services start mysql', hotkey: 'Ctrl+Shift+M' },
      { id: 'db-mysql-stop', name: '⏹️ Stop Service MySQL', command: 'brew services stop mysql', hotkey: 'Ctrl+Shift+S' },
      { id: 'db-mysql-connect', name: '🔗 Connect Database MySQL', command: 'mysql -u [username] -p [database]', inputs: { username: 'user', database: 'testdb' }, hotkey: 'Ctrl+Shift+C' },
      { id: 'db-mongodb-start', name: '🍃 Start Service MongoDB', command: 'brew services start mongodb-community', hotkey: 'Ctrl+Shift+M' },
      { id: 'db-mongodb-stop', name: '⏹️ Stop Service MongoDB', command: 'brew services stop mongodb-community', hotkey: 'Ctrl+Shift+S' },
      { id: 'db-mongodb-connect', name: '🔗 Connect Database MongoDB', command: 'mongosh [database]', inputs: { database: 'testdb' }, hotkey: 'Ctrl+Shift+C' },
      { id: 'db-redis-start', name: '🔴 Start Service Redis', command: 'brew services start redis', hotkey: 'Ctrl+Shift+R' },
      { id: 'db-redis-stop', name: '⏹️ Stop Service Redis', command: 'brew services stop redis', hotkey: 'Ctrl+Shift+S' },
      { id: 'db-redis-connect', name: '🔗 Connect Database Redis', command: 'redis-cli', hotkey: 'Ctrl+Shift+C' },
      { id: 'db-sqlite-create', name: '📱 Create Database SQLite', command: 'sqlite3 [database_name].db', inputs: { database_name: 'testdb' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'db-migrate-run', name: '🔄 Run Migrations npm', command: 'npm run migrate', hotkey: 'Ctrl+Shift+M' },
      { id: 'db-migrate-rollback', name: '↩️ Rollback Migrations npm', command: 'npm run migrate:rollback', hotkey: 'Ctrl+Shift+R' },
      { id: 'db-seed-run', name: '🌱 Run Seeders npm', command: 'npm run seed', hotkey: 'Ctrl+Shift+S' },
      { id: 'db-backup', name: '💾 Backup Database PostgreSQL', command: 'pg_dump [database] > backup_[date].sql', inputs: { database: 'testdb' }, hotkey: 'Ctrl+Shift+B' },
      { id: 'db-restore', name: '📥 Restore Database PostgreSQL', command: 'psql [database] < [backup_file]', inputs: { database: 'testdb', backup_file: 'backup.sql' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'db-optimize', name: '⚡ Optimize Database PostgreSQL', command: 'VACUUM ANALYZE;', hotkey: 'Ctrl+Shift+O' },
      { id: 'db-monitor', name: '📊 Monitor Database PostgreSQL', command: 'SELECT * FROM pg_stat_activity;', hotkey: 'Ctrl+Shift+M' }
    ]
  },
  {
    id: 'cloud-category',
    name: 'Cloud',
    description: 'Cloud infrastructure and deployment tools',
    category: 'Cloud',
    icon: '☁️',
    tags: ['cloud', 'aws', 'azure', 'gcp', 'kubernetes', 'docker'],
      commands: [
      { id: 'cloud-aws-cli', name: '☁️ Configure AWS CLI', command: 'aws configure', hotkey: 'Ctrl+Shift+A' },
      { id: 'cloud-aws-s3', name: '📦 List Buckets AWS S3', command: 'aws s3 ls', hotkey: 'Ctrl+Shift+S' },
      { id: 'cloud-aws-ec2', name: '🖥️ List Instances AWS EC2', command: 'aws ec2 describe-instances', hotkey: 'Ctrl+Shift+E' },
      { id: 'cloud-aws-lambda', name: '⚡ List Functions AWS Lambda', command: 'aws lambda list-functions', hotkey: 'Ctrl+Shift+L' },
      { id: 'cloud-azure-cli', name: '☁️ Login Azure CLI', command: 'az login', hotkey: 'Ctrl+Shift+A' },
      { id: 'cloud-azure-vm', name: '🖥️ List VMs Azure', command: 'az vm list', hotkey: 'Ctrl+Shift+V' },
      { id: 'cloud-azure-storage', name: '📦 List Storage Azure', command: 'az storage account list', hotkey: 'Ctrl+Shift+S' },
      { id: 'cloud-gcp-cli', name: '☁️ Configure GCP CLI', command: 'gcloud auth login', hotkey: 'Ctrl+Shift+G' },
      { id: 'cloud-gcp-compute', name: '🖥️ List Instances GCP', command: 'gcloud compute instances list', hotkey: 'Ctrl+Shift+C' },
      { id: 'cloud-gcp-storage', name: '📦 List Buckets GCP', command: 'gsutil ls', hotkey: 'Ctrl+Shift+S' },
      { id: 'cloud-terraform-init', name: '🏗️ Init Terraform', command: 'terraform init', hotkey: 'Ctrl+Shift+I' },
      { id: 'cloud-terraform-plan', name: '📋 Plan Terraform', command: 'terraform plan', hotkey: 'Ctrl+Shift+P' },
      { id: 'cloud-terraform-apply', name: '🚀 Apply Terraform', command: 'terraform apply', hotkey: 'Ctrl+Shift+A' },
      { id: 'cloud-terraform-destroy', name: '🗑️ Destroy Terraform', command: 'terraform destroy', hotkey: 'Ctrl+Shift+D' },
      { id: 'cloud-k8s-pods', name: '📦 Get Pods Kubernetes', command: 'kubectl get pods -A', hotkey: 'Ctrl+Shift+K' },
      { id: 'cloud-k8s-services', name: '🌐 Get Services Kubernetes', command: 'kubectl get services -A', hotkey: 'Ctrl+Shift+S' },
      { id: 'cloud-k8s-nodes', name: '🖥️ Get Nodes Kubernetes', command: 'kubectl get nodes', hotkey: 'Ctrl+Shift+N' },
      { id: 'cloud-docker-images', name: '🐳 List Images Docker', command: 'docker images', hotkey: 'Ctrl+Shift+I' },
      { id: 'cloud-docker-containers', name: '📦 List Containers Docker', command: 'docker ps -a', hotkey: 'Ctrl+Shift+C' }
    ]
  },
  {
    id: 'security-category',
    name: 'Security',
    description: 'Security tools and penetration testing',
    category: 'Security',
    icon: '🔒',
    tags: ['security', 'pentest', 'vulnerability', 'audit', 'encryption'],
      commands: [
      { id: 'security-nmap-scan', name: '🔍 Scan Network nmap', command: 'nmap -sS -sV [target]', inputs: { target: '192.168.1.1' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'security-nmap-vuln', name: '🔍 Scan Vulnerabilities nmap', command: 'nmap --script vuln [target]', inputs: { target: '192.168.1.1' }, hotkey: 'Ctrl+Shift+V' },
      { id: 'security-nikto-scan', name: '🌐 Scan Web Server nikto', command: 'nikto -h [url]', inputs: { url: 'http://example.com' }, hotkey: 'Ctrl+Shift+K' },
      { id: 'security-dirb-scan', name: '📁 Scan Directories dirb', command: 'dirb [url]', inputs: { url: 'http://example.com' }, hotkey: 'Ctrl+Shift+D' },
      { id: 'security-dirb-wordlist', name: '📁 Scan with Wordlist dirb', command: 'dirb [url] /usr/share/wordlists/dirb/common.txt', inputs: { url: 'http://example.com' }, hotkey: 'Ctrl+Shift+G' },
      { id: 'security-sqlmap-scan', name: '🗄️ Scan Database sqlmap', command: 'sqlmap -u [url] --dbs', inputs: { url: 'http://example.com' }, hotkey: 'Ctrl+Shift+Q' },
      { id: 'security-hydra-brute', name: '🔓 Brute Force hydra', command: 'hydra -l [username] -P [wordlist] [target] [service]', inputs: { username: 'admin', wordlist: 'wordlist.txt', target: '192.168.1.1', service: 'ssh' }, hotkey: 'Ctrl+Shift+H' },
      { id: 'security-john-crack', name: '🔓 Crack Passwords john', command: 'john [hash_file]', inputs: { hash_file: 'hashes.txt' }, hotkey: 'Ctrl+Shift+J' },
      { id: 'security-hashcat-crack', name: '🔓 Crack with Hashcat', command: 'hashcat -m [hash_type] [hash_file] [wordlist]', inputs: { hash_type: '0', hash_file: 'hashes.txt', wordlist: 'wordlist.txt' }, hotkey: 'Ctrl+Shift+C' },
      { id: 'security-metasploit', name: '🎯 Start Framework Metasploit', command: 'msfconsole', hotkey: 'Ctrl+Shift+M' },
      { id: 'security-burp-suite', name: '🕷️ Start Proxy Burp Suite', command: 'burpsuite', hotkey: 'Ctrl+Shift+B' },
      { id: 'security-wireshark-capture', name: '📡 Capture Traffic Wireshark', command: 'tshark -i [interface] -w capture.pcap', inputs: { interface: 'eth0' }, hotkey: 'Ctrl+Shift+T' },
      { id: 'security-openssl-test', name: '�� Test SSL openssl', command: 'openssl s_client -connect [host]:[port] -servername [host]', inputs: { host: 'localhost', port: '443' }, hotkey: 'Ctrl+Shift+L' },
      { id: 'security-nuclei-scan', name: '🎯 Scan Vulnerabilities nuclei', command: 'nuclei -u [url]', inputs: { url: 'http://example.com' }, hotkey: 'Ctrl+Shift+U' },
      { id: 'security-subfinder', name: '🔍 Find Subdomains subfinder', command: 'subfinder -d [domain]', inputs: { domain: 'example.com' }, hotkey: 'Ctrl+Shift+F' },
      { id: 'security-amass-scan', name: '🕸️ Scan Subdomains amass', command: 'amass enum -d [domain]', inputs: { domain: 'example.com' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'security-masscan-scan', name: '⚡ Scan Ports masscan', command: 'masscan [target] -p [ports]', inputs: { target: '192.168.1.0/24', ports: '80,443,22' }, hotkey: 'Ctrl+Shift+P' },
      { id: 'security-responder', name: '🎣 Poison LLMNR Responder', command: 'responder -I [interface]', inputs: { interface: 'eth0' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'security-smb-enum', name: '📁 Enumerate SMB smbmap', command: 'smbmap -H [target] -smb2support', inputs: { target: '192.168.1.1' }, hotkey: 'Ctrl+Shift+I' }
    ]
  },
  {
    id: 'testing-category',
    name: 'Testing',
    description: 'Software testing and quality assurance tools',
    category: 'Testing',
    icon: '🧪',
    tags: ['testing', 'qa', 'automation', 'selenium', 'cypress', 'jest'],
      commands: [
      { id: 'testing-jest-run', name: '⚡ Run Tests Jest', command: 'npm test', hotkey: 'Ctrl+Shift+J' },
      { id: 'testing-jest-watch', name: '👁️ Watch Tests Jest', command: 'npm test -- --watch', hotkey: 'Ctrl+Shift+W' },
      { id: 'testing-jest-coverage', name: '📊 Coverage Report Jest', command: 'npm test -- --coverage', hotkey: 'Ctrl+Shift+C' },
      { id: 'testing-cypress-open', name: '🌐 Open Browser Cypress', command: 'npx cypress open', hotkey: 'Ctrl+Shift+O' },
      { id: 'testing-cypress-run', name: '🏃 Run Tests Cypress', command: 'npx cypress run', hotkey: 'Ctrl+Shift+R' },
      { id: 'testing-selenium', name: '🤖 Run Tests Selenium', command: 'python -m pytest tests/ --driver chrome', hotkey: 'Ctrl+Shift+S' },
      { id: 'testing-playwright', name: '🎭 Run Tests Playwright', command: 'npx playwright test', hotkey: 'Ctrl+Shift+P' },
      { id: 'testing-playwright-ui', name: '🎭 UI Mode Playwright', command: 'npx playwright test --ui', hotkey: 'Ctrl+Shift+U' },
      { id: 'testing-vitest', name: '⚡ Run Tests Vitest', command: 'npm run test:unit', hotkey: 'Ctrl+Shift+V' },
      { id: 'testing-storybook', name: '📚 Start Storybook', command: 'npm run storybook', hotkey: 'Ctrl+Shift+K' },
      { id: 'testing-lighthouse', name: '📊 Audit Performance Lighthouse', command: 'lighthouse [url] --output html --output-path lighthouse-report.html', inputs: { url: 'http://example.com' }, hotkey: 'Ctrl+Shift+L' },
      { id: 'testing-puppeteer', name: '🎪 Run E2E Tests Puppeteer', command: 'node tests/puppeteer.js', hotkey: 'Ctrl+Shift+E' },
      { id: 'testing-postman', name: '📮 Run Collection Postman', command: 'newman run [collection.json]', inputs: { collection: 'collection.json' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'testing-k6-load', name: '📈 Load Test k6', command: 'k6 run [script.js]', inputs: { script: 'test.js' }, hotkey: 'Ctrl+Shift+K' },
      { id: 'testing-artillery', name: '💥 Load Test Artillery', command: 'artillery run [config.yml]', inputs: { config: 'config.yml' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'testing-jmeter', name: '📊 Load Test JMeter', command: 'jmeter -n -t [test.jmx] -l results.jtl', inputs: { test: 'test.jmx' }, hotkey: 'Ctrl+Shift+M' },
      { id: 'testing-selenium-grid', name: '🌐 Start Grid Selenium', command: 'java -jar selenium-server.jar hub', hotkey: 'Ctrl+Shift+G' },
      { id: 'testing-appium', name: '📱 Start Server Appium', command: 'appium', hotkey: 'Ctrl+Shift+A' },
      { id: 'testing-detox', name: '🧪 Run E2E Tests Detox', command: 'detox test', hotkey: 'Ctrl+Shift+D' },
      { id: 'testing-testcafe', name: '☕ Run Tests TestCafe', command: 'testcafe chrome tests/', hotkey: 'Ctrl+Shift+T' }
    ]
  }
];

export function getTemplatesByCategory(category?: string): Template[] {
  if (!category) return templates;
  return templates.filter(template => template.category.toLowerCase().replace(' ', '-') === category);
}

export function searchTemplates(query: string): Template[] {
  const lowercaseQuery = query.toLowerCase();
  return templates.filter(template => 
    template.name.toLowerCase().includes(lowercaseQuery) ||
    template.description.toLowerCase().includes(lowercaseQuery) ||
    template.tags.some(tag => tag.toLowerCase().includes(lowercaseQuery))
  );
}

export function getTemplateById(id: string): Template | undefined {
  return templates.find(template => template.id === id);
} 