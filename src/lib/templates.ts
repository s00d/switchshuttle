import type { Config } from '../types';

export interface Template {
  id: string;
  name: string;
  description: string;
  category: string;
  icon: string;
  config: Config;
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
    config: {
      terminal: 'iterm',
      launch_in: 'new_tab',
      theme: 'Homebrew',
      title: 'Development',
      menu_hotkey: 'Ctrl+Shift+D',
      commands: [
        {
          id: 'dev-setup-project',
          name: '🚀 Setup New Project',
          command: 'mkdir [project_name] && cd [project_name] && npm init -y',
          inputs: {
            project_name: 'Enter project name:'
          },
          hotkey: 'Ctrl+Shift+N'
        },
        {
          id: 'dev-install-deps',
          name: '📦 Install Dependencies',
          command: 'npm install [package_name]',
          inputs: {
            package_name: 'Enter package name:'
          },
          hotkey: 'Ctrl+Shift+I'
        },
        {
          id: 'dev-run-script',
          name: '▶️ Run Script',
          command: 'npm run [script_name]',
          inputs: {
            script_name: 'Enter script name:'
          },
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'dev-build',
          name: '📦 Build Project',
          command: 'npm run build',
          hotkey: 'Ctrl+Shift+B'
        },
        {
          id: 'dev-test',
          name: '🧪 Run Tests',
          command: 'npm test',
          hotkey: 'Ctrl+Shift+T'
        },
        {
          id: 'dev-lint',
          name: '🔍 Lint Code',
          command: 'npm run lint',
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'dev-format',
          name: '✨ Format Code',
          command: 'npm run format',
          hotkey: 'Ctrl+Shift+F'
        },
        {
          id: 'dev-watch',
          name: '👁️ Watch Mode',
          command: 'npm run dev',
          hotkey: 'Ctrl+Shift+W'
        },
        {
          id: 'dev-debug',
          name: '🐛 Debug Mode',
          command: 'npm run debug',
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'dev-clean',
          name: '🧹 Clean Build',
          command: 'rm -rf node_modules package-lock.json && npm install',
          hotkey: 'Ctrl+Shift+C'
        },
        {
          id: 'dev-update',
          name: '🔄 Update Dependencies',
          command: 'npm update',
          hotkey: 'Ctrl+Shift+U'
        },
        {
          id: 'dev-audit',
          name: '🔍 Security Audit',
          command: 'npm audit',
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'dev-outdated',
          name: '📋 Check Outdated',
          command: 'npm outdated',
          hotkey: 'Ctrl+Shift+O'
        },
        {
          id: 'dev-publish',
          name: '📤 Publish Package',
          command: 'npm publish',
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'dev-version',
          name: '🏷️ Bump Version',
          command: 'npm version [version_type]',
          inputs: {
            version_type: 'Enter version type (patch, minor, major):'
          },
          hotkey: 'Ctrl+Shift+V'
        },
        {
          id: 'dev-git-init',
          name: '📚 Initialize Git',
          command: 'git init && git add . && git commit -m "Initial commit"',
          hotkey: 'Ctrl+Shift+G'
        },
        {
          id: 'dev-readme',
          name: '📖 Create README',
          command: 'touch README.md',
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'dev-license',
          name: '📄 Add License',
          command: 'touch LICENSE',
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'dev-gitignore',
          name: '🚫 Create .gitignore',
          command: 'touch .gitignore',
          hotkey: 'Ctrl+Shift+I'
        },
        {
          id: 'dev-env',
          name: '🌍 Environment Setup',
          command: 'touch .env.example && touch .env',
          hotkey: 'Ctrl+Shift+E'
        }
      ]
    }
  },
  {
    id: 'devops-category',
    name: 'DevOps',
    description: 'DevOps tools and practices for continuous integration and deployment',
    category: 'DevOps',
    icon: '🔧',
    tags: ['devops', 'ci-cd', 'automation', 'deployment'],
    config: {
      terminal: 'iterm',
      launch_in: 'new_tab',
      theme: 'Homebrew',
      title: 'DevOps',
      menu_hotkey: 'Ctrl+Shift+O',
      commands: [
        {
          id: 'devops-docker-build',
          name: '🐳 Build Docker Image',
          command: 'docker build -t [image_name] .',
          inputs: {
            image_name: 'Enter image name:'
          },
          hotkey: 'Ctrl+Shift+B'
        },
        {
          id: 'devops-docker-run',
          name: '▶️ Run Docker Container',
          command: 'docker run -d --name [container_name] -p [port]:[port] [image_name]',
          inputs: {
            container_name: 'Enter container name:',
            port: 'Enter port:',
            image_name: 'Enter image name:'
          },
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'devops-docker-compose',
          name: '🚀 Docker Compose Up',
          command: 'docker-compose up -d',
          hotkey: 'Ctrl+Shift+U'
        },
        {
          id: 'devops-docker-stop',
          name: '⏹️ Docker Compose Down',
          command: 'docker-compose down',
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'devops-k8s-pods',
          name: '📦 Get Kubernetes Pods',
          command: 'kubectl get pods -A',
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'devops-k8s-services',
          name: '🌐 Get Kubernetes Services',
          command: 'kubectl get services -A',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'devops-k8s-apply',
          name: '📋 Apply Kubernetes Manifest',
          command: 'kubectl apply -f [manifest.yaml]',
          inputs: {
            manifest: 'Enter manifest file path:'
          },
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'devops-k8s-logs',
          name: '📋 View Kubernetes Logs',
          command: 'kubectl logs [pod_name] -f',
          inputs: {
            pod_name: 'Enter pod name:'
          },
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'devops-k8s-exec',
          name: '🔧 Execute in Kubernetes Pod',
          command: 'kubectl exec -it [pod_name] -- /bin/bash',
          inputs: {
            pod_name: 'Enter pod name:'
          },
          hotkey: 'Ctrl+Shift+E'
        },
        {
          id: 'devops-terraform-init',
          name: '🏗️ Initialize Terraform',
          command: 'terraform init',
          hotkey: 'Ctrl+Shift+I'
        },
        {
          id: 'devops-terraform-plan',
          name: '📋 Terraform Plan',
          command: 'terraform plan',
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'devops-terraform-apply',
          name: '🚀 Terraform Apply',
          command: 'terraform apply',
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'devops-terraform-destroy',
          name: '🗑️ Terraform Destroy',
          command: 'terraform destroy',
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'devops-ansible-playbook',
          name: '📜 Run Ansible Playbook',
          command: 'ansible-playbook [playbook.yml]',
          inputs: {
            playbook: 'Enter playbook file:'
          },
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'devops-jenkins-build',
          name: '🔨 Trigger Jenkins Build',
          command: 'curl -X POST [jenkins_url]/job/[job_name]/build',
          inputs: {
            jenkins_url: 'Enter Jenkins URL:',
            job_name: 'Enter job name:'
          },
          hotkey: 'Ctrl+Shift+J'
        },
        {
          id: 'devops-gitlab-ci',
          name: '🔄 Run GitLab CI',
          command: 'gitlab-ci-local',
          hotkey: 'Ctrl+Shift+G'
        },
        {
          id: 'devops-github-actions',
          name: '⚡ GitHub Actions Workflow',
          command: 'gh workflow run [workflow_name]',
          inputs: {
            workflow_name: 'Enter workflow name:'
          },
          hotkey: 'Ctrl+Shift+H'
        },
        {
          id: 'devops-monitoring',
          name: '📊 Check System Metrics',
          command: 'htop',
          hotkey: 'Ctrl+Shift+M'
        },
        {
          id: 'devops-logs',
          name: '📋 View Application Logs',
          command: 'tail -f [log_file]',
          inputs: {
            log_file: 'Enter log file path:'
          },
          hotkey: 'Ctrl+Shift+L'
        }
      ]
    }
  },
  {
    id: 'frontend-category',
    name: 'Frontend',
    description: 'Frontend development frameworks and tools',
    category: 'Frontend',
    icon: '🎨',
    tags: ['frontend', 'ui', 'javascript', 'css', 'html'],
    config: {
      terminal: 'iterm',
      launch_in: 'new_tab',
      theme: 'Homebrew',
      title: 'Frontend',
      menu_hotkey: 'Ctrl+Shift+F',
      commands: [
        {
          id: 'frontend-react-create',
          name: '⚛️ Create React App',
          command: 'npx create-react-app [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'frontend-vue-create',
          name: '💚 Create Vue App',
          command: 'npm create vue@latest [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+V'
        },
        {
          id: 'frontend-angular-create',
          name: '🅰️ Create Angular App',
          command: 'ng new [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'frontend-svelte-create',
          name: '⚡ Create Svelte App',
          command: 'npm create svelte@latest [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'frontend-nextjs-create',
          name: '⚡ Create Next.js App',
          command: 'npx create-next-app@latest [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+N'
        },
        {
          id: 'frontend-dev-server',
          name: '▶️ Start Dev Server',
          command: 'npm run dev',
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'frontend-build',
          name: '📦 Build for Production',
          command: 'npm run build',
          hotkey: 'Ctrl+Shift+B'
        },
        {
          id: 'frontend-preview',
          name: '👁️ Preview Build',
          command: 'npm run preview',
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'frontend-test',
          name: '🧪 Run Tests',
          command: 'npm test',
          hotkey: 'Ctrl+Shift+T'
        },
        {
          id: 'frontend-lint',
          name: '🔍 Lint Code',
          command: 'npm run lint',
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'frontend-format',
          name: '✨ Format Code',
          command: 'npm run format',
          hotkey: 'Ctrl+Shift+F'
        },
        {
          id: 'frontend-component',
          name: '🧩 Create Component',
          command: 'touch src/components/[component_name].jsx',
          inputs: {
            component_name: 'Enter component name:'
          },
          hotkey: 'Ctrl+Shift+C'
        },
        {
          id: 'frontend-page',
          name: '📄 Create Page',
          command: 'touch src/pages/[page_name].jsx',
          inputs: {
            page_name: 'Enter page name:'
          },
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'frontend-style',
          name: '🎨 Create Style File',
          command: 'touch src/styles/[style_name].css',
          inputs: {
            style_name: 'Enter style name:'
          },
          hotkey: 'Ctrl+Shift+Y'
        },
        {
          id: 'frontend-asset',
          name: '📁 Create Asset',
          command: 'mkdir -p public/[asset_type]',
          inputs: {
            asset_type: 'Enter asset type (images, fonts, icons):'
          },
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'frontend-router',
          name: '🛣️ Setup Router',
          command: 'npm install react-router-dom',
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'frontend-state',
          name: '📊 Setup State Management',
          command: 'npm install @reduxjs/toolkit react-redux',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'frontend-ui',
          name: '🎨 Setup UI Library',
          command: 'npm install @mui/material @emotion/react @emotion/styled',
          hotkey: 'Ctrl+Shift+U'
        },
        {
          id: 'frontend-storybook',
          name: '📚 Setup Storybook',
          command: 'npx storybook@latest init',
          hotkey: 'Ctrl+Shift+K'
        },
        {
          id: 'frontend-pwa',
          name: '📱 PWA Setup',
          command: 'npm install workbox-webpack-plugin',
          hotkey: 'Ctrl+Shift+P'
        }
      ]
    }
  },
  {
    id: 'backend-category',
    name: 'Backend',
    description: 'Backend development frameworks and server-side tools',
    category: 'Backend',
    icon: '⚙️',
    tags: ['backend', 'api', 'server', 'database'],
    config: {
      terminal: 'iterm',
      launch_in: 'new_tab',
      theme: 'Homebrew',
      title: 'Backend',
      menu_hotkey: 'Ctrl+Shift+B',
      commands: [
        {
          id: 'backend-node-init',
          name: '🟢 Initialize Node.js Project',
          command: 'npm init -y',
          hotkey: 'Ctrl+Shift+N'
        },
        {
          id: 'backend-express-create',
          name: '🚀 Create Express App',
          command: 'npm install express cors helmet morgan',
          hotkey: 'Ctrl+Shift+E'
        },
        {
          id: 'backend-fastify-create',
          name: '⚡ Create Fastify App',
          command: 'npm install fastify',
          hotkey: 'Ctrl+Shift+F'
        },
        {
          id: 'backend-koa-create',
          name: '🌊 Create Koa App',
          command: 'npm install koa @koa/router',
          hotkey: 'Ctrl+Shift+K'
        },
        {
          id: 'backend-nest-create',
          name: '🪺 Create NestJS App',
          command: 'npm install -g @nestjs/cli && nest new [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+N'
        },
        {
          id: 'backend-django-create',
          name: '🐍 Create Django Project',
          command: 'django-admin startproject [project_name]',
          inputs: {
            project_name: 'Enter project name:'
          },
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'backend-flask-create',
          name: '🍶 Create Flask App',
          command: 'mkdir [app_name] && cd [app_name] && touch app.py',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+F'
        },
        {
          id: 'backend-fastapi-create',
          name: '⚡ Create FastAPI App',
          command: 'pip install fastapi uvicorn',
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'backend-spring-create',
          name: '🍃 Create Spring Boot App',
          command: 'curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o [app_name].zip',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'backend-laravel-create',
          name: '🦋 Create Laravel App',
          command: 'composer create-project laravel/laravel [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'backend-rails-create',
          name: '💎 Create Rails App',
          command: 'rails new [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'backend-go-create',
          name: '🐹 Create Go App',
          command: 'mkdir [app_name] && cd [app_name] && go mod init [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+G'
        },
        {
          id: 'backend-rust-create',
          name: '🦀 Create Rust App',
          command: 'cargo new [app_name]',
          inputs: {
            app_name: 'Enter app name:'
          },
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'backend-server-start',
          name: '▶️ Start Server',
          command: 'npm start',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'backend-server-dev',
          name: '🔄 Start Dev Server',
          command: 'npm run dev',
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'backend-test',
          name: '🧪 Run Tests',
          command: 'npm test',
          hotkey: 'Ctrl+Shift+T'
        },
        {
          id: 'backend-lint',
          name: '🔍 Lint Code',
          command: 'npm run lint',
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'backend-build',
          name: '📦 Build Project',
          command: 'npm run build',
          hotkey: 'Ctrl+Shift+B'
        },
        {
          id: 'backend-migrate',
          name: '🗄️ Run Migrations',
          command: 'npm run migrate',
          hotkey: 'Ctrl+Shift+M'
        },
        {
          id: 'backend-seed',
          name: '🌱 Seed Database',
          command: 'npm run seed',
          hotkey: 'Ctrl+Shift+S'
        }
      ]
    }
  },
  {
    id: 'database-category',
    name: 'Database',
    description: 'Database management and operations',
    category: 'Database',
    icon: '🗄️',
    tags: ['database', 'sql', 'nosql', 'orm'],
    config: {
      terminal: 'iterm',
      launch_in: 'new_tab',
      theme: 'Homebrew',
      title: 'Database',
      menu_hotkey: 'Ctrl+Shift+D',
      commands: [
        {
          id: 'db-postgres-start',
          name: '🐘 Start PostgreSQL',
          command: 'brew services start postgresql',
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'db-postgres-stop',
          name: '⏹️ Stop PostgreSQL',
          command: 'brew services stop postgresql',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'db-postgres-connect',
          name: '🔗 Connect to PostgreSQL',
          command: 'psql -U [username] -d [database]',
          inputs: {
            username: 'Enter username:',
            database: 'Enter database name:'
          },
          hotkey: 'Ctrl+Shift+C'
        },
        {
          id: 'db-mysql-start',
          name: '🐬 Start MySQL',
          command: 'brew services start mysql',
          hotkey: 'Ctrl+Shift+M'
        },
        {
          id: 'db-mysql-stop',
          name: '⏹️ Stop MySQL',
          command: 'brew services stop mysql',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'db-mysql-connect',
          name: '🔗 Connect to MySQL',
          command: 'mysql -u [username] -p [database]',
          inputs: {
            username: 'Enter username:',
            database: 'Enter database name:'
          },
          hotkey: 'Ctrl+Shift+C'
        },
        {
          id: 'db-mongodb-start',
          name: '🍃 Start MongoDB',
          command: 'brew services start mongodb-community',
          hotkey: 'Ctrl+Shift+M'
        },
        {
          id: 'db-mongodb-stop',
          name: '⏹️ Stop MongoDB',
          command: 'brew services stop mongodb-community',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'db-mongodb-connect',
          name: '🔗 Connect to MongoDB',
          command: 'mongosh [database]',
          inputs: {
            database: 'Enter database name:'
          },
          hotkey: 'Ctrl+Shift+C'
        },
        {
          id: 'db-redis-start',
          name: '🔴 Start Redis',
          command: 'brew services start redis',
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'db-redis-stop',
          name: '⏹️ Stop Redis',
          command: 'brew services stop redis',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'db-redis-connect',
          name: '🔗 Connect to Redis',
          command: 'redis-cli',
          hotkey: 'Ctrl+Shift+C'
        },
        {
          id: 'db-sqlite-create',
          name: '📱 Create SQLite Database',
          command: 'sqlite3 [database_name].db',
          inputs: {
            database_name: 'Enter database name:'
          },
          hotkey: 'Ctrl+Shift+N'
        },
        {
          id: 'db-migrate-run',
          name: '🔄 Run Migrations',
          command: 'npm run migrate',
          hotkey: 'Ctrl+Shift+M'
        },
        {
          id: 'db-migrate-rollback',
          name: '↩️ Rollback Migrations',
          command: 'npm run migrate:rollback',
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'db-seed-run',
          name: '🌱 Run Seeders',
          command: 'npm run seed',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'db-backup',
          name: '💾 Backup Database',
          command: 'pg_dump [database] > backup_[date].sql',
          inputs: {
            database: 'Enter database name:'
          },
          hotkey: 'Ctrl+Shift+B'
        },
        {
          id: 'db-restore',
          name: '📥 Restore Database',
          command: 'psql [database] < [backup_file]',
          inputs: {
            database: 'Enter database name:',
            backup_file: 'Enter backup file path:'
          },
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'db-optimize',
          name: '⚡ Optimize Database',
          command: 'VACUUM ANALYZE;',
          hotkey: 'Ctrl+Shift+O'
        },
        {
          id: 'db-monitor',
          name: '📊 Monitor Database',
          command: 'SELECT * FROM pg_stat_activity;',
          hotkey: 'Ctrl+Shift+M'
        }
      ]
    }
  },
  {
    id: 'cloud-category',
    name: 'Cloud',
    description: 'Cloud infrastructure and platform management',
    category: 'Cloud',
    icon: '☁️',
    tags: ['cloud', 'aws', 'azure', 'gcp', 'kubernetes', 'docker'],
    config: {
      terminal: 'iterm',
      launch_in: 'new_tab',
      theme: 'Homebrew',
      title: 'Cloud',
      menu_hotkey: 'Ctrl+Shift+C',
      commands: [
        {
          id: 'cloud-aws-login',
          name: '🔐 AWS Login',
          command: 'aws configure',
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'cloud-aws-s3-ls',
          name: '📦 List S3 Buckets',
          command: 'aws s3 ls',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'cloud-aws-ec2-ls',
          name: '🖥️ List EC2 Instances',
          command: 'aws ec2 describe-instances --query "Reservations[*].Instances[*].[InstanceId,State.Name,InstanceType,PublicIpAddress]" --output table',
          hotkey: 'Ctrl+Shift+E'
        },
        {
          id: 'cloud-aws-lambda-ls',
          name: '⚡ List Lambda Functions',
          command: 'aws lambda list-functions --query "Functions[*].[FunctionName,Runtime,CodeSize]" --output table',
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'cloud-aws-cloudformation',
          name: '🏗️ List CloudFormation Stacks',
          command: 'aws cloudformation list-stacks --query "StackSummaries[*].[StackName,StackStatus]" --output table',
          hotkey: 'Ctrl+Shift+F'
        },
        {
          id: 'cloud-azure-login',
          name: '🔐 Azure Login',
          command: 'az login',
          hotkey: 'Ctrl+Shift+Z'
        },
        {
          id: 'cloud-azure-vm-ls',
          name: '🖥️ List Azure VMs',
          command: 'az vm list --query "[].{Name:name,ResourceGroup:resourceGroup,Status:powerState}" --output table',
          hotkey: 'Ctrl+Shift+V'
        },
        {
          id: 'cloud-azure-storage',
          name: '💾 List Storage Accounts',
          command: 'az storage account list --query "[].{Name:name,ResourceGroup:resourceGroup,Location:location}" --output table',
          hotkey: 'Ctrl+Shift+T'
        },
        {
          id: 'cloud-gcp-login',
          name: '🔐 GCP Login',
          command: 'gcloud auth login',
          hotkey: 'Ctrl+Shift+G'
        },
        {
          id: 'cloud-gcp-instances',
          name: '🖥️ List GCP Instances',
          command: 'gcloud compute instances list',
          hotkey: 'Ctrl+Shift+I'
        },
        {
          id: 'cloud-gcp-buckets',
          name: '📦 List GCS Buckets',
          command: 'gsutil ls',
          hotkey: 'Ctrl+Shift+B'
        },
        {
          id: 'cloud-terraform-init',
          name: '🏗️ Terraform Init',
          command: 'terraform init',
          hotkey: 'Ctrl+Shift+T'
        },
        {
          id: 'cloud-terraform-plan',
          name: '📋 Terraform Plan',
          command: 'terraform plan',
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'cloud-terraform-apply',
          name: '🚀 Terraform Apply',
          command: 'terraform apply',
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'cloud-terraform-destroy',
          name: '🗑️ Terraform Destroy',
          command: 'terraform destroy',
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'cloud-k8s-pods',
          name: '📦 Get K8s Pods',
          command: 'kubectl get pods -A',
          hotkey: 'Ctrl+Shift+K'
        },
        {
          id: 'cloud-k8s-services',
          name: '🌐 Get K8s Services',
          command: 'kubectl get services -A',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'cloud-k8s-nodes',
          name: '🖥️ Get K8s Nodes',
          command: 'kubectl get nodes',
          hotkey: 'Ctrl+Shift+N'
        },
        {
          id: 'cloud-docker-images',
          name: '🐳 List Docker Images',
          command: 'docker images',
          hotkey: 'Ctrl+Shift+I'
        },
        {
          id: 'cloud-docker-containers',
          name: '📦 List Docker Containers',
          command: 'docker ps -a',
          hotkey: 'Ctrl+Shift+C'
        }
      ]
    }
  },
  {
    id: 'security-category',
    name: 'Security',
    description: 'Security tools and penetration testing',
    category: 'Security',
    icon: '🔒',
    tags: ['security', 'pentest', 'vulnerability', 'audit', 'encryption'],
    config: {
      terminal: 'iterm',
      launch_in: 'new_tab',
      theme: 'Homebrew',
      title: 'Security',
      menu_hotkey: 'Ctrl+Shift+S',
      commands: [
        {
          id: 'security-nmap-scan',
          name: '🔍 Nmap Network Scan',
          command: 'nmap -sS -sV [target]',
          inputs: {
            target: 'Enter target IP or hostname'
          },
          hotkey: 'Ctrl+Shift+N'
        },
        {
          id: 'security-nmap-vuln',
          name: '🔍 Nmap Vulnerability Scan',
          command: 'nmap --script vuln [target]',
          inputs: {
            target: 'Enter target IP or hostname'
          },
          hotkey: 'Ctrl+Shift+V'
        },
        {
          id: 'security-nikto-scan',
          name: '🌐 Nikto Web Scan',
          command: 'nikto -h [url]',
          inputs: {
            url: 'Enter target URL'
          },
          hotkey: 'Ctrl+Shift+K'
        },
        {
          id: 'security-dirb-scan',
          name: '📁 Dirb Directory Scan',
          command: 'dirb [url]',
          inputs: {
            url: 'Enter target URL'
          },
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'security-gobuster-scan',
          name: '🔍 Gobuster Directory Scan',
          command: 'gobuster dir -u [url] -w /usr/share/wordlists/dirb/common.txt',
          inputs: {
            url: 'Enter target URL'
          },
          hotkey: 'Ctrl+Shift+G'
        },
        {
          id: 'security-sqlmap-scan',
          name: '🗄️ SQLMap Database Scan',
          command: 'sqlmap -u [url] --dbs',
          inputs: {
            url: 'Enter target URL'
          },
          hotkey: 'Ctrl+Shift+Q'
        },
        {
          id: 'security-hydra-brute',
          name: '🔑 Hydra Password Brute',
          command: 'hydra -l [username] -P [wordlist] [target] [service]',
          inputs: {
            username: 'Enter username',
            wordlist: 'Enter wordlist path',
            target: 'Enter target IP',
            service: 'Enter service (ssh, ftp, etc)'
          },
          hotkey: 'Ctrl+Shift+H'
        },
        {
          id: 'security-john-crack',
          name: '🔓 John Password Cracking',
          command: 'john [hash_file]',
          inputs: {
            hash_file: 'Enter hash file path'
          },
          hotkey: 'Ctrl+Shift+J'
        },
        {
          id: 'security-hashcat-crack',
          name: '⚡ Hashcat GPU Cracking',
          command: 'hashcat -m [hash_type] [hash_file] [wordlist]',
          inputs: {
            hash_type: 'Enter hash type (0=MD5, 100=SHA1)',
            hash_file: 'Enter hash file path',
            wordlist: 'Enter wordlist path'
          },
          hotkey: 'Ctrl+Shift+C'
        },
        {
          id: 'security-metasploit',
          name: '🎯 Metasploit Framework',
          command: 'msfconsole',
          hotkey: 'Ctrl+Shift+M'
        },
        {
          id: 'security-burp-suite',
          name: '🕷️ Burp Suite',
          command: 'burpsuite',
          hotkey: 'Ctrl+Shift+B'
        },
        {
          id: 'security-wireshark',
          name: '📡 Wireshark Network Analysis',
          command: 'wireshark',
          hotkey: 'Ctrl+Shift+W'
        },
        {
          id: 'security-tcpdump',
          name: '📡 TCPDump Packet Capture',
          command: 'tcpdump -i [interface] -w capture.pcap',
          inputs: {
            interface: 'Enter network interface'
          },
          hotkey: 'Ctrl+Shift+T'
        },
        {
          id: 'security-ssl-check',
          name: '🔐 SSL Certificate Check',
          command: 'openssl s_client -connect [host]:[port] -servername [host]',
          inputs: {
            host: 'Enter hostname',
            port: 'Enter port (default: 443)'
          },
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'security-nuclei-scan',
          name: '🎯 Nuclei Vulnerability Scan',
          command: 'nuclei -u [url]',
          inputs: {
            url: 'Enter target URL'
          },
          hotkey: 'Ctrl+Shift+U'
        },
        {
          id: 'security-subfinder',
          name: '🔍 Subfinder Subdomain Enum',
          command: 'subfinder -d [domain]',
          inputs: {
            domain: 'Enter target domain'
          },
          hotkey: 'Ctrl+Shift+F'
        },
        {
          id: 'security-amass-scan',
          name: '🕸️ Amass Subdomain Scan',
          command: 'amass enum -d [domain]',
          inputs: {
            domain: 'Enter target domain'
          },
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'security-masscan-scan',
          name: '⚡ Masscan Port Scan',
          command: 'masscan [target] -p [ports]',
          inputs: {
            target: 'Enter target IP range',
            ports: 'Enter ports (1-65535)'
          },
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'security-responder',
          name: '🎣 Responder LLMNR Poison',
          command: 'responder -I [interface]',
          inputs: {
            interface: 'Enter network interface'
          },
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'security-impacket',
          name: '🔐 Impacket Tools',
          command: 'impacket-ntlmrelayx -t [target] -smb2support',
          inputs: {
            target: 'Enter target IP'
          },
          hotkey: 'Ctrl+Shift+I'
        }
      ]
    }
  },
  {
    id: 'testing-category',
    name: 'Testing',
    description: 'Software testing and quality assurance tools',
    category: 'Testing',
    icon: '🧪',
    tags: ['testing', 'qa', 'automation', 'selenium', 'cypress', 'jest'],
    config: {
      terminal: 'iterm',
      launch_in: 'new_tab',
      theme: 'Homebrew',
      title: 'Testing',
      menu_hotkey: 'Ctrl+Shift+T',
      commands: [
        {
          id: 'testing-jest-run',
          name: '⚡ Run Jest Tests',
          command: 'npm test',
          hotkey: 'Ctrl+Shift+J'
        },
        {
          id: 'testing-jest-watch',
          name: '👁️ Jest Watch Mode',
          command: 'npm test -- --watch',
          hotkey: 'Ctrl+Shift+W'
        },
        {
          id: 'testing-jest-coverage',
          name: '📊 Jest Coverage Report',
          command: 'npm test -- --coverage',
          hotkey: 'Ctrl+Shift+C'
        },
        {
          id: 'testing-cypress-open',
          name: '🌐 Cypress Open',
          command: 'npx cypress open',
          hotkey: 'Ctrl+Shift+O'
        },
        {
          id: 'testing-cypress-run',
          name: '🏃 Cypress Run',
          command: 'npx cypress run',
          hotkey: 'Ctrl+Shift+R'
        },
        {
          id: 'testing-selenium',
          name: '🤖 Selenium WebDriver',
          command: 'python -m pytest tests/ --driver chrome',
          hotkey: 'Ctrl+Shift+S'
        },
        {
          id: 'testing-playwright',
          name: '🎭 Playwright Test',
          command: 'npx playwright test',
          hotkey: 'Ctrl+Shift+P'
        },
        {
          id: 'testing-playwright-ui',
          name: '🎭 Playwright UI Mode',
          command: 'npx playwright test --ui',
          hotkey: 'Ctrl+Shift+U'
        },
        {
          id: 'testing-vitest',
          name: '⚡ Vitest Test Runner',
          command: 'npm run test:unit',
          hotkey: 'Ctrl+Shift+V'
        },
        {
          id: 'testing-storybook',
          name: '📚 Storybook',
          command: 'npm run storybook',
          hotkey: 'Ctrl+Shift+K'
        },
        {
          id: 'testing-lighthouse',
          name: '💡 Lighthouse Audit',
          command: 'npx lighthouse [url] --output html --output-path ./lighthouse-report.html',
          inputs: {
            url: 'Enter URL to audit'
          },
          hotkey: 'Ctrl+Shift+L'
        },
        {
          id: 'testing-puppeteer',
          name: '🎪 Puppeteer E2E',
          command: 'node tests/puppeteer.js',
          hotkey: 'Ctrl+Shift+E'
        },
        {
          id: 'testing-postman',
          name: '📮 Postman Collection',
          command: 'newman run [collection.json]',
          inputs: {
            collection: 'Enter collection file path'
          },
          hotkey: 'Ctrl+Shift+N'
        },
        {
          id: 'testing-k6-load',
          name: '📈 K6 Load Test',
          command: 'k6 run [script.js]',
          inputs: {
            script: 'Enter K6 script path'
          },
          hotkey: 'Ctrl+Shift+K'
        },
        {
          id: 'testing-artillery',
          name: '💥 Artillery Load Test',
          command: 'artillery run [config.yml]',
          inputs: {
            config: 'Enter Artillery config path'
          },
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'testing-jmeter',
          name: '📊 Apache JMeter',
          command: 'jmeter -n -t [test.jmx] -l results.jtl',
          inputs: {
            test: 'Enter JMeter test file path'
          },
          hotkey: 'Ctrl+Shift+M'
        },
        {
          id: 'testing-selenium-grid',
          name: '🌐 Selenium Grid',
          command: 'java -jar selenium-server.jar hub',
          hotkey: 'Ctrl+Shift+G'
        },
        {
          id: 'testing-appium',
          name: '📱 Appium Mobile Test',
          command: 'appium',
          hotkey: 'Ctrl+Shift+A'
        },
        {
          id: 'testing-detox',
          name: '🧪 Detox E2E',
          command: 'detox test',
          hotkey: 'Ctrl+Shift+D'
        },
        {
          id: 'testing-testcafe',
          name: '☕ TestCafe',
          command: 'testcafe chrome tests/',
          hotkey: 'Ctrl+Shift+T'
        }
      ]
    }
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