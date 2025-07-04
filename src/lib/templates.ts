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
      { id: 'dev-env', name: '🌍 Create .env files', command: 'touch .env.example && touch .env', hotkey: 'Ctrl+Shift+E' },
      { id: 'dev-tauri-create', name: '⚡ Create App Tauri', command: 'cargo create-tauri-app [app_name]', inputs: { app_name: 'my-tauri-app' }, hotkey: 'Ctrl+Shift+T' },
      { id: 'dev-tauri-dev', name: '🔄 Dev Mode Tauri', command: 'cargo tauri dev', hotkey: 'Ctrl+Shift+D' },
      { id: 'dev-tauri-autostart', name: '🚀 Add Autostart Tauri', command: 'cargo tauri add autostart', hotkey: 'Ctrl+Shift+A' },
      { id: 'dev-tauri-build', name: '📦 Build App Tauri', command: 'cargo tauri build', hotkey: 'Ctrl+Shift+B' },
      { id: 'dev-git-clone', name: '📥 Clone Repository Git', command: 'git clone [repository_url]', inputs: { repository_url: 'https://github.com/user/repo.git' }, hotkey: 'Ctrl+Shift+C' },
      { id: 'dev-git-pull', name: '⬇️ Pull Changes Git', command: 'git pull origin main', hotkey: 'Ctrl+Shift+P' },
      { id: 'dev-git-push', name: '⬆️ Push Changes Git', command: 'git push origin main', hotkey: 'Ctrl+Shift+U' },
      { id: 'dev-git-branch', name: '🌿 Create Branch Git', command: 'git checkout -b [branch_name]', inputs: { branch_name: 'feature/new-feature' }, hotkey: 'Ctrl+Shift+B' },
      { id: 'dev-git-merge', name: '🔀 Merge Branch Git', command: 'git merge [branch_name]', inputs: { branch_name: 'feature/new-feature' }, hotkey: 'Ctrl+Shift+M' },
      { id: 'dev-yarn-install', name: '📦 Install Dependencies Yarn', command: 'yarn install', hotkey: 'Ctrl+Shift+Y' },
      { id: 'dev-yarn-add', name: '➕ Add Package Yarn', command: 'yarn add [package_name]', inputs: { package_name: 'lodash' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'dev-pnpm-install', name: '📦 Install Dependencies pnpm', command: 'pnpm install', hotkey: 'Ctrl+Shift+P' },
      { id: 'dev-bun-install', name: '📦 Install Dependencies Bun', command: 'bun install', hotkey: 'Ctrl+Shift+B' },
      { id: 'dev-docker-compose', name: '🐳 Compose Up Docker', command: 'docker-compose up -d', hotkey: 'Ctrl+Shift+U' },
      { id: 'dev-docker-run', name: '▶️ Run Container Docker', command: 'docker run -d --name [container_name] [image_name]', inputs: { container_name: 'my-container', image_name: 'nginx' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'dev-docker-exec', name: '🔧 Exec Container Docker', command: 'docker exec -it [container_name] /bin/bash', inputs: { container_name: 'my-container' }, hotkey: 'Ctrl+Shift+E' },
      { id: 'dev-npx-run', name: '⚡ Run Package npx', command: 'npx [package_name]', inputs: { package_name: 'create-react-app' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'dev-npm-audit-fix', name: '🔧 Fix Security npm', command: 'npm audit fix', hotkey: 'Ctrl+Shift+F' },
      { id: 'dev-npm-outdated', name: '📋 Check Outdated npm', command: 'npm outdated', hotkey: 'Ctrl+Shift+O' }
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
      { id: 'frontend-pwa', name: '📱 Install PWA Workbox', command: 'npm install workbox-webpack-plugin', hotkey: 'Ctrl+Shift+P' },
      { id: 'frontend-vite-create', name: '⚡ Create App Vite', command: 'npm create vite@latest [app_name]', inputs: { app_name: 'my-vite-app' }, hotkey: 'Ctrl+Shift+V' },
      { id: 'frontend-sveltekit-create', name: '⚡ Create App SvelteKit', command: 'npm create svelte@latest [app_name]', inputs: { app_name: 'my-sveltekit-app' }, hotkey: 'Ctrl+Shift+S' },
      { id: 'frontend-astro-create', name: '🚀 Create App Astro', command: 'npm create astro@latest [app_name]', inputs: { app_name: 'my-astro-app' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'frontend-remix-create', name: '⚡ Create App Remix', command: 'npx create-remix@latest [app_name]', inputs: { app_name: 'my-remix-app' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'frontend-nuxt-create', name: '🟢 Create App Nuxt', command: 'npx nuxi@latest init [app_name]', inputs: { app_name: 'my-nuxt-app' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'frontend-gatsby-create', name: '📚 Create App Gatsby', command: 'npx gatsby new [app_name]', inputs: { app_name: 'my-gatsby-app' }, hotkey: 'Ctrl+Shift+G' },
      { id: 'frontend-tailwind-init', name: '🎨 Init Tailwind CSS', command: 'npx tailwindcss init', hotkey: 'Ctrl+Shift+T' }
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
      { id: 'backend-laravel-version', name: '🦋 Create App Laravel Version', command: 'composer create-project laravel/laravel:[version] [app_name]', inputs: { version: '9.0', app_name: 'my-laravel-app' }, hotkey: 'Ctrl+Shift+V' },
      { id: 'backend-rails-create', name: '💎 Create App Rails', command: 'rails new [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'backend-go-create', name: '🐹 Create App Go', command: 'mkdir [app_name] && cd [app_name] && go mod init [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+G' },
      { id: 'backend-rust-create', name: '🦀 Create App Rust', command: 'cargo new [app_name]', inputs: { app_name: 'my-app' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'backend-server-start', name: '▶️ Start Server npm', command: 'npm start', hotkey: 'Ctrl+Shift+S' },
      { id: 'backend-server-dev', name: '🔄 Start Dev Server npm', command: 'npm run dev', hotkey: 'Ctrl+Shift+D' },
      { id: 'backend-test', name: '🧪 Run Tests npm', command: 'npm test', hotkey: 'Ctrl+Shift+T' },
      { id: 'backend-lint', name: '🔍 Lint Code npm', command: 'npm run lint', hotkey: 'Ctrl+Shift+L' },
      { id: 'backend-build', name: '📦 Build Project npm', command: 'npm run build', hotkey: 'Ctrl+Shift+B' },
      { id: 'backend-migrate', name: '🗄️ Run Migrations npm', command: 'npm run migrate', hotkey: 'Ctrl+Shift+M' },
      { id: 'backend-seed', name: '🌱 Seed Database npm', command: 'npm run seed', hotkey: 'Ctrl+Shift+S' },
      { id: 'backend-fastapi-install', name: '⚡ Install FastAPI', command: 'pip install fastapi uvicorn', hotkey: 'Ctrl+Shift+F' },
      { id: 'backend-django-rest', name: '🔄 Install Django REST', command: 'pip install djangorestframework', hotkey: 'Ctrl+Shift+R' },
      { id: 'backend-flask-sqlalchemy', name: '🗄️ Install Flask SQLAlchemy', command: 'pip install flask-sqlalchemy', hotkey: 'Ctrl+Shift+S' },
      { id: 'backend-flask-cors', name: '🌐 Install Flask CORS', command: 'pip install flask-cors', hotkey: 'Ctrl+Shift+C' },
      { id: 'backend-express-cors', name: '🌐 Install Express CORS', command: 'npm install cors', hotkey: 'Ctrl+Shift+C' },
      { id: 'backend-express-helmet', name: '🛡️ Install Express Helmet', command: 'npm install helmet', hotkey: 'Ctrl+Shift+H' },
      { id: 'backend-prisma-init', name: '🗄️ Init Prisma ORM', command: 'npx prisma init', hotkey: 'Ctrl+Shift+P' },
      { id: 'backend-typeorm-install', name: '🗄️ Install TypeORM', command: 'npm install typeorm', hotkey: 'Ctrl+Shift+T' }
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
      { id: 'db-monitor', name: '📊 Monitor Database PostgreSQL', command: 'SELECT * FROM pg_stat_activity;', hotkey: 'Ctrl+Shift+M' },
      { id: 'db-prisma-push', name: '🔄 Push Schema Prisma', command: 'npx prisma db push', hotkey: 'Ctrl+Shift+P' },
      { id: 'db-prisma-studio', name: '🎨 Open Prisma Studio', command: 'npx prisma studio', hotkey: 'Ctrl+Shift+S' },
      { id: 'db-sequelize-init', name: '🗄️ Init Sequelize CLI', command: 'npx sequelize-cli init', hotkey: 'Ctrl+Shift+I' },
      { id: 'db-mongoose-install', name: '🍃 Install Mongoose', command: 'npm install mongoose', hotkey: 'Ctrl+Shift+M' },
      { id: 'db-redis-ping', name: '🏓 Ping Redis', command: 'redis-cli ping', hotkey: 'Ctrl+Shift+P' },
      { id: 'db-redis-flush', name: '🗑️ Flush Redis', command: 'redis-cli flushall', hotkey: 'Ctrl+Shift+F' }
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
      { id: 'cloud-docker-containers', name: '📦 List Containers Docker', command: 'docker ps -a', hotkey: 'Ctrl+Shift+C' },
      { id: 'cloud-github-repo', name: '📚 Create Repository GitHub', command: 'gh repo create [repo_name] --public', inputs: { repo_name: 'my-repo' }, hotkey: 'Ctrl+Shift+G' },
      { id: 'cloud-github-pr', name: '🔀 Create PR GitHub', command: 'gh pr create --title "[title]"', inputs: { title: 'Add new feature' }, hotkey: 'Ctrl+Shift+P' },
      { id: 'cloud-aws-sync', name: '🔄 Sync S3 AWS', command: 'aws s3 sync [local_path] s3://[bucket_name]', inputs: { local_path: './dist', bucket_name: 'my-bucket' }, hotkey: 'Ctrl+Shift+S' },
      { id: 'cloud-terraform-validate', name: '✅ Validate Terraform', command: 'terraform validate', hotkey: 'Ctrl+Shift+V' },
      { id: 'cloud-ansible-playbook', name: '📜 Run Playbook Ansible', command: 'ansible-playbook -i inventory [playbook.yml]', inputs: { playbook: 'playbook.yml' }, hotkey: 'Ctrl+Shift+A' }
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
      { id: 'security-openssl-test', name: '🔓 Test SSL openssl', command: 'openssl s_client -connect [host]:[port] -servername [host]', inputs: { host: 'localhost', port: '443' }, hotkey: 'Ctrl+Shift+L' },
      { id: 'security-nuclei-scan', name: '🎯 Scan Vulnerabilities nuclei', command: 'nuclei -u [url]', inputs: { url: 'http://example.com' }, hotkey: 'Ctrl+Shift+U' },
      { id: 'security-subfinder', name: '🔍 Find Subdomains subfinder', command: 'subfinder -d [domain]', inputs: { domain: 'example.com' }, hotkey: 'Ctrl+Shift+F' },
      { id: 'security-amass-scan', name: '🕸️ Scan Subdomains amass', command: 'amass enum -d [domain]', inputs: { domain: 'example.com' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'security-masscan-scan', name: '⚡ Scan Ports masscan', command: 'masscan [target] -p [ports]', inputs: { target: '192.168.1.0/24', ports: '80,443,22' }, hotkey: 'Ctrl+Shift+P' },
      { id: 'security-responder', name: '🎣 Poison LLMNR Responder', command: 'responder -I [interface]', inputs: { interface: 'eth0' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'security-smb-enum', name: '📁 Enumerate SMB smbmap', command: 'smbmap -H [target] -smb2support', inputs: { target: '192.168.1.1' }, hotkey: 'Ctrl+Shift+I' },
      { id: 'security-ssl-cert', name: '🔐 Generate SSL Certificate', command: 'openssl req -newkey rsa:2048 -keyout key.pem -out cert.pem', hotkey: 'Ctrl+Shift+S' },
      { id: 'security-ssh-keygen', name: '🔑 Generate SSH Key', command: 'ssh-keygen -t rsa -b 4096 -C "[email]"', inputs: { email: 'user@example.com' }, hotkey: 'Ctrl+Shift+K' },
      { id: 'security-certbot-nginx', name: '🔒 SSL Certificate Certbot', command: 'certbot --nginx -d [domain]', inputs: { domain: 'example.com' }, hotkey: 'Ctrl+Shift+C' }
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
      { id: 'testing-testcafe', name: '☕ Run Tests TestCafe', command: 'testcafe chrome tests/', hotkey: 'Ctrl+Shift+T' },
      { id: 'testing-cypress-headed', name: '👁️ Run Cypress Headed', command: 'npx cypress run --headed', hotkey: 'Ctrl+Shift+H' },
      { id: 'testing-playwright-install', name: '🎭 Install Playwright', command: 'npx playwright install', hotkey: 'Ctrl+Shift+I' },
      { id: 'testing-vitest-ui', name: '⚡ UI Mode Vitest', command: 'npm run test:ui', hotkey: 'Ctrl+Shift+U' }
    ]
  },
  {
    id: 'utility-category',
    name: 'Utility',
    description: 'System utilities and file operations',
    category: 'Utility',
    icon: '🔧',
    tags: ['utility', 'system', 'file', 'network', 'compression'],
    commands: [
      { id: 'utility-htop', name: '📊 System Monitor htop', command: 'htop', hotkey: 'Ctrl+Shift+H' },
      { id: 'utility-disk-usage', name: '💾 Disk Usage df', command: 'df -h', hotkey: 'Ctrl+Shift+D' },
      { id: 'utility-dir-size', name: '📁 Directory Size du', command: 'du -sh [directory]', inputs: { directory: '.' }, hotkey: 'Ctrl+Shift+S' },
      { id: 'utility-process-list', name: '📋 Process List ps', command: 'ps aux', hotkey: 'Ctrl+Shift+P' },
      { id: 'utility-ping', name: '🏓 Ping Host', command: 'ping [host]', inputs: { host: 'google.com' }, hotkey: 'Ctrl+Shift+P' },
      { id: 'utility-curl', name: '🌐 HTTP Request curl', command: 'curl [url]', inputs: { url: 'https://httpbin.org/get' }, hotkey: 'Ctrl+Shift+C' },
      { id: 'utility-wget', name: '⬇️ Download File wget', command: 'wget [url]', inputs: { url: 'https://example.com/file.zip' }, hotkey: 'Ctrl+Shift+W' },
      { id: 'utility-nslookup', name: '🔍 DNS Lookup nslookup', command: 'nslookup [domain]', inputs: { domain: 'google.com' }, hotkey: 'Ctrl+Shift+N' },
      { id: 'utility-find', name: '🔍 Find Files', command: 'find [path] -name "[pattern]"', inputs: { path: '.', pattern: '*.js' }, hotkey: 'Ctrl+Shift+F' },
      { id: 'utility-grep', name: '🔍 Search Text grep', command: 'grep -r "[pattern]" [path]', inputs: { pattern: 'function', path: '.' }, hotkey: 'Ctrl+Shift+G' },
      { id: 'utility-ripgrep', name: '⚡ Fast Search ripgrep', command: 'rg "[pattern]" [path]', inputs: { pattern: 'function', path: '.' }, hotkey: 'Ctrl+Shift+R' },
      { id: 'utility-sed', name: '✏️ Text Replace sed', command: 'sed "s/[old]/[new]/g" [file]', inputs: { old: 'old-text', new: 'new-text', file: 'file.txt' }, hotkey: 'Ctrl+Shift+S' },
      { id: 'utility-awk', name: '📊 Text Processing awk', command: 'awk "{print $1}" [file]', inputs: { file: 'data.txt' }, hotkey: 'Ctrl+Shift+A' },
      { id: 'utility-tar-compress', name: '📦 Compress Archive tar', command: 'tar -czf [archive.tar.gz] [directory]', inputs: { archive: 'backup.tar.gz', directory: 'my-folder' }, hotkey: 'Ctrl+Shift+C' },
      { id: 'utility-zip-compress', name: '📦 Compress ZIP', command: 'zip -r [archive.zip] [directory]', inputs: { archive: 'backup.zip', directory: 'my-folder' }, hotkey: 'Ctrl+Shift+Z' },
      { id: 'utility-unzip', name: '📦 Extract ZIP', command: 'unzip [archive.zip]', inputs: { archive: 'backup.zip' }, hotkey: 'Ctrl+Shift+U' },
      { id: 'utility-scp', name: '🔐 Secure Copy scp', command: 'scp [local_file] [user]@[host]:[remote_path]', inputs: { local_file: 'file.txt', user: 'user', host: 'example.com', remote_path: '/home/user/' }, hotkey: 'Ctrl+Shift+S' }
    ]
  }
];

export function getTemplatesByCategory(category?: string): Template[] {
  if (!category) return templates;
  return templates.filter(template => template.category.toLowerCase().replace(' ', '-') === category);
}

export function searchTemplates(query: string): Template[] {
  const lowercaseQuery = query.toLowerCase();
  return templates.filter(template => {
    // Поиск по названию шаблона, описанию и тегам
    const templateMatch = template.name.toLowerCase().includes(lowercaseQuery) ||
                         template.description.toLowerCase().includes(lowercaseQuery) ||
                         template.tags.some(tag => tag.toLowerCase().includes(lowercaseQuery));
    
    // Поиск по названиям команд
    const commandMatch = template.commands.some(command => 
      command.name.toLowerCase().includes(lowercaseQuery)
    );
    
    return templateMatch || commandMatch;
  });
}

export function getTemplateById(id: string): Template | undefined {
  return templates.find(template => template.id === id);
} 