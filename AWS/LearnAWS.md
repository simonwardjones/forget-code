# learnAWS

<sub>[Back to ForgetCode](../README.md)</sub>
<!-- MarkdownTOC autolink="true" bracket="round" indent="    "-->

- [General](#general)
    - [Launch a Linux Virtual Machine \(Through web Console\)](#launch-a-linux-virtual-machine-through-web-console)

<!-- /MarkdownTOC -->

# General

## Launch a Linux Virtual Machine (Through web Console)

>> Amazon Elastic Compute Cloud (EC2) is the Amazon Web Service you use to create and run virtual machines in the cloud (we call these virtual machines 'instances'). 

1. Select Launch Amazon EC2 Instance
2. Configure Instance
	* Select the AMI (Amazon Machine Image), this specifies the OS (operating system), application server and applications e.g. `Amazon Linux AMI`
	* Select the instance type and hit `Review and Launch`
	* Review settings and then hit `Launch`
	* Create a key pair and safely save the private key. e.g. call it `MyKeyPair`
	* Hit `Launch`
	* Note the Public DNS down. My example is `Public DNS: ec2-54-245-188-214.us-west-2.compute.amazonaws.com` or the IP `54.245.188.214`
3. Connect to your Instance
	* Use chmod to make mykeypair private ```chmod 400 ~/.ssh/mykeypair.pem```
	* Use ssh to connect to your instance
	```ssh -i ~/.ssh/MyKeyPair.pem ec2-user@54.245.188.214``` 
	or
	```ssh -i ~/.ssh/MyKeyPair.pem ec2-user@ec2-54-245-188-214.us-west-2.compute.amazonaws.com```
	
	When prompted say yes!
