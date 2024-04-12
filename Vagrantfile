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
  config.vm.provision "file", source: "./.env", destination: "~/.env"

  config.vm.define "local" do |config|
    config.vm.box = "bento/ubuntu-22.04"
    config.vm.provider :libvirt do |domain|
      domain.memory = 2048
      domain.cpus = 4
    end
    config.vm.provision "file", source: "./docker-compose-monitoring.yml", destination: "~/docker-compose.yml"
    config.vm.provision "file", source: "./prometheus.yaml", destination: "~/prometheus.yaml"
    config.vm.network "forwarded_port", guest: 3000, host: 3000
    config.vm.network "forwarded_port", guest: 9090, host: 9090
    
    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/home/vagrant/docker-compose.yml", run: "always"
  end
  # Manually had to patch my digital ocean plugin, by removing the {}
  # https://discuss.hashicorp.com/t/vagrant-digital-ocean-plugin-broken-with-2-3-6/54132
  config.vm.define "droplet" do |config|
    config.vm.provision "file", source: "./docker-compose.yml", destination: "~/docker-compose.yml"
    config.vm.network "forwarded_port", guest: 5001, host: 5001
    config.vm.network "forwarded_port", guest: 5000, host: 5000

    config.vm.provider :digital_ocean do |provider, override|
      override.ssh.private_key_path = "~/.ssh/id_ed25519"
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

    config.vm.define "worker" do |config|
      config.vm.network "forwarded_port", guest: 5001, host: 5001
      config.vm.network "forwarded_port", guest: 5000, host: 5000
  
      config.vm.provider :digital_ocean do |provider, override|
        override.ssh.private_key_path = "~/.ssh/id_ed25519"
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
    end

    # Wait for apt to be ready 
    config.vm.provision "shell", inline: <<-SHELL
        apt-get -o DPkg::Lock::Timeout=120 update -qq -y
      SHELL
    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/root/docker-compose.yml", run: "always"
  end

  config.vm.define "monitoring" do |config|
    config.vm.provision "file", source: "./docker-compose-monitoring.yml", destination: "~/docker-compose.yml"
    config.vm.provision "file", source: "./prometheus.yaml", destination: "~/prometheus.yaml"
    config.vm.network "forwarded_port", guest: 3000, host: 3000
    config.vm.network "forwarded_port", guest: 9090, host: 9090

    config.vm.provider :digital_ocean do |provider, override|
      override.ssh.private_key_path = "~/.ssh/id_ed25519"
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
        apt-get -o DPkg::Lock::Timeout=120 update -qq -y
      SHELL
    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/root/docker-compose.yml", run: "always"
  end

  config.vm.define "postgres" do |config|
    config.vm.provision "file", source: "./docker-compose.db.yml", destination: "~/docker-compose.yml"
    config.vm.network "forwarded_port", guest: 5432, host: 5432
    config.ssh.private_key_path = "~/.ssh/vagrant"

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
        apt-get -o DPkg::Lock::Timeout=120 update -qq -y
      SHELL
    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/root/docker-compose.yml", run: "always"
  end

end