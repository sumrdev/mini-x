# -*- mode: ruby -*-
# vi: set ft=ruby :

# Start local vm with "vagrant up local"
# Start digital ocean droplet with "vagrant up droplet"

# Plugins: vagrant-digitalocean vagrant-docker-compose vagrant-env vagrant-reload vagrant-scp 
# And either: vagrant-vbguest OR vagrant-libvirt for local vms
# use VAGRANT_DISABLE_STRICT_DEPENDENCY_ENFORCEMENT=1 if installed version is too new

Vagrant.configure("2") do |config|
  config.env.enable
  config.vm.synced_folder '.', '/vagrant', disabled: true
  #config.vm.provision "file", source: "./.env", destination: "~/.env"
  config.vm.provision "file", source: "./filebeat.yml", destination: "~/filebeat.yml"
  config.vm.provision "shell", env: {
      "URL" => ENV['ES_URL'],
      "USERNAME" => ENV['ES_USERNAME'], 
      "PASSWORD" => ENV['ES_PASSWORD'],
      "PROTOCOL" => ENV['ES_PROTOCOL']
      }, inline: <<-SHELL
        echo "ES_URL=$URL" > ~/.env
        echo "ES_USERNAME=$USERNAME" >> ~/.env
        echo "ES_PASSWORD=$PASSWORD" >> ~/.env
        echo "ES_PROTOCOL=$PROTOCOL" >> ~/.env
    SHELL

  ## Main droplet
  # Manually had to patch my digital ocean plugin, by removing the {}
  # https://discuss.hashicorp.com/t/vagrant-digital-ocean-plugin-broken-with-2-3-6/54132
  config.vm.define "droplet" do |config|
    config.vm.provision "file", source: "./docker-compose.yml", destination: "~/docker-compose.yml"
    config.vm.provision "file", source: "./docker-compose-swag.yml", destination: "~/docker-compose-swag.yml"
    config.vm.provision "file", source: "./swag.conf", destination: "~/swag/nginx/site-confs/default.conf"
    config.vm.network "forwarded_port", guest: 5001, host: 5001
    config.vm.network "forwarded_port", guest: 5000, host: 5000

    config.vm.provider :digital_ocean do |provider, override|
      override.ssh.private_key_path = "~/.ssh/ssh_key"
      override.vm.box = 'digital_ocean'
      override.nfs.functional = false
      override.vm.allowed_synced_folder_types = :rsync
      provider.token = ENV["DIGITAL_OCEAN_TOKEN"]
      provider.image = 'ubuntu-22-04-x64'
      provider.region = 'fra1'
      provider.size = 's-1vcpu-1gb'
      provider.backups_enabled = false
      provider.private_networking = false
      provider.ipv6 = false
      provider.monitoring = false
    end
    # Wait for apt to be ready 
    config.vm.provision "shell", inline: <<-SHELL

        echo "Sleep"
        sleep 5
        echo "Woke up"
      SHELL
    config.vm.provision "shell", env: {"DATABASE_URL" => ENV['DATABASE_URL']}, inline: <<-SHELL
          echo "DATABASE_URL=$DATABASE_URL" >> ~/.env
      SHELL
    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/root/docker-compose.yml", run: "always"
    config.vm.provision :docker_compose, yml: "/root/docker-compose-swag.yml", run: "always"
  end

  ## Worker droplet1
  config.vm.define "worker1" do |config|
    config.vm.network "forwarded_port", guest: 5001, host: 5001
    config.vm.network "forwarded_port", guest: 5000, host: 5000

    config.vm.provider :digital_ocean do |provider, override|
      override.ssh.private_key_path = "~/.ssh/ssh_key"
      override.vm.box = 'digital_ocean'
      override.nfs.functional = false
      override.vm.allowed_synced_folder_types = :rsync
      provider.token = ENV["DIGITAL_OCEAN_TOKEN"]
      provider.image = 'ubuntu-22-04-x64'
      provider.region = 'fra1'
      provider.size = 's-1vcpu-1gb'
      provider.backups_enabled = false
      provider.private_networking = false
      provider.ipv6 = false
      provider.monitoring = false
    end
    # Wait for apt to be ready 
    config.vm.provision "shell", inline: <<-SHELL
        echo "Sleep"
        sleep 5
        echo "Woke up"
      SHELL
    config.vm.provision "shell", env: {"DATABASE_URL" => ENV['DATABASE_URL']}, inline: <<-SHELL
          echo "DATABASE_URL=$DATABASE_URL" >> ~/.env
      SHELL
    config.vm.provision :docker
  end

  ## Worker droplet1
  config.vm.define "worker2" do |config|
    config.vm.network "forwarded_port", guest: 5001, host: 5001
    config.vm.network "forwarded_port", guest: 5000, host: 5000

    config.vm.provider :digital_ocean do |provider, override|
      override.ssh.private_key_path = "~/.ssh/ssh_key"
      override.vm.box = 'digital_ocean'
      override.nfs.functional = false
      override.vm.allowed_synced_folder_types = :rsync
      provider.token = ENV["DIGITAL_OCEAN_TOKEN"]
      provider.image = 'ubuntu-22-04-x64'
      provider.region = 'fra1'
      provider.size = 's-1vcpu-1gb'
      provider.backups_enabled = false
      provider.private_networking = false
      provider.ipv6 = false
      provider.monitoring = false
    end
    # Wait for apt to be ready 
    config.vm.provision "shell", inline: <<-SHELL
        echo "Sleep"
        sleep 5
        echo "Woke up"
      SHELL
    config.vm.provision :docker
  end

  config.vm.define "monitoring" do |config|
    config.vm.provision "file", source: "./docker-compose-monitoring.yml", destination: "~/docker-compose.yml"
    config.vm.provision "file", source: "./prometheus.yaml", destination: "~/prometheus.yaml"
    config.vm.network "forwarded_port", guest: 3000, host: 3000
    config.vm.network "forwarded_port", guest: 9090, host: 9090

    config.vm.provider :digital_ocean do |provider, override|
      override.ssh.private_key_path = "~/.ssh/ssh_key"
      override.vm.box = 'digital_ocean'
      override.nfs.functional = false
      override.vm.allowed_synced_folder_types = :rsync
      provider.token = ENV["DIGITAL_OCEAN_TOKEN"]
      provider.image = 'ubuntu-22-04-x64'
      provider.region = 'fra1'
      provider.size = 's-1vcpu-1gb'
      provider.backups_enabled = false
      provider.private_networking = false
      provider.ipv6 = false
      provider.monitoring = false
    end
    # Wait for apt to be ready 
    config.vm.provision "shell", inline: <<-SHELL
        echo "Sleep"
        sleep 5
        echo "Woke up"
      SHELL
    config.vm.provision "shell", env: {"DATABASE_URL" => ENV['DATABASE_URL']}, inline: <<-SHELL
          echo "DATABASE_URL=$DATABASE_URL" >> ~/.env
      SHELL
    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/root/docker-compose.yml", run: "always"
  end

  ## Logging droplet
  config.vm.define "logging" do |config|
    config.vm.provision "file", source: "./docker-compose-logging.yml", destination: "~/docker-compose.yml"
    config.vm.provision "file", source: "./nginx.conf", destination: "~/nginx.conf"
    config.vm.network "forwarded_port", guest: 5601, host: 5601
    config.vm.network "forwarded_port", guest: 9200, host: 9200
    config.vm.network "forwarded_port", guest: 8881, host: 8881
    config.vm.network "forwarded_port", guest: 8882, host: 8882

    config.vm.provider :digital_ocean do |provider, override|
      override.ssh.private_key_path = '~/.ssh/ssh_key'
      override.vm.box = 'digital_ocean'
      override.nfs.functional = false
      override.vm.allowed_synced_folder_types = :rsync
      provider.token = ENV["DIGITAL_OCEAN_TOKEN"]
      provider.image = 'ubuntu-22-04-x64'
      provider.region = 'fra1'
      provider.size = 's-2vcpu-4gb' #Elasticsearch needs alot of memory it seems
      provider.backups_enabled = false
      provider.private_networking = false
      provider.ipv6 = false
      provider.monitoring = false
    end
    # Wait for apt to be ready 
    config.vm.provision "shell", inline: <<-SHELL
        echo "Sleep"
        sleep 5
        echo "Woke up"
      SHELL
    config.vm.provision "shell", env: {
      "USERNAME" => ENV['ES_USERNAME'], 
      "PASSWORD" => ENV['ES_PASSWORD'],
      }, inline: <<-SHELL
        apt-get update -y
        apt-get install -y apache2-utils
        htpasswd -cb .htpasswd "$USERNAME" "$PASSWORD"
      SHELL
    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/root/docker-compose.yml", run: "always"
  end

  config.vm.define "postgres" do |config|
    config.vm.provision "file", source: "./docker-compose-db.yml", destination: "~/docker-compose.yml"
    config.vm.network "forwarded_port", guest: 5432, host: 5432
    config.ssh.private_key_path = "~/.ssh/ssh_key"

    config.vm.provider :digital_ocean do |provider, override|
      override.vm.box = 'digital_ocean'
      override.nfs.functional = false
      override.vm.allowed_synced_folder_types = :rsync
      provider.token = ENV["DIGITAL_OCEAN_TOKEN"]
      provider.image = 'ubuntu-22-04-x64'
      provider.region = 'fra1'
      provider.size = 's-1vcpu-1gb'
      provider.backups_enabled = false
      provider.private_networking = false
      provider.ipv6 = false
      provider.monitoring = false
    end
    # Wait for apt to be ready 
    config.vm.provision "shell", inline: <<-SHELL
        echo "Sleep"
        sleep 300
        echo "Woke up"
      SHELL
    config.vm.provision "shell", env: { "PASSWORD" => ENV['POSTGRES_PASSWORD'] }, inline: <<-SHELL
          echo "POSTGRES_PASSWORD=$PASSWORD" >> ~/.env
      SHELL
    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/root/docker-compose.yml", run: "always"
  end

end