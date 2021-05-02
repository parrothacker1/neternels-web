import psycopg2
import requests
class SQL:
	def __init__(self):
		DATABASE_URL = os.environ['DATABASE_URL']
		self.conn = psycopg2.connect(DATABASE_URL, sslmode='require')
		self.cur=self.conn.cursor()
	def login(self,bool):
		cur=self.cur
		self.bool=bool
		cur.execute("SELECT bool FROM login")
		if self.bool == "True":
			if "True" in cur.fetchall()[0]:
				cur.execute("UPDATE login set bool='True' where bool='True'")
			else:
				cur.execute("UPDATE login set bool='True' where bool='False'")
		else:
			if "True" in cur.fetchall()[0]:
				cur.execute("UPDATE login set bool='False' where bool='True'")
			else:
				cur.execute("UPDATE login set bool='False' where bool='False'")
		self.conn.commit()
	def ret_log(self):
		cur=self.cur
		cur.execute("SELECT bool FROM login")
		return cur.fetchall()
	def passwd(self):
		cur=self.cur
		cur.execute("SELECT * from passwd")
		return cur.fetchall[0][0]
	def get_members(self):
		cur=self.cur
		cur.execute("SELECT * FROM members;")
		return cur.fetchall()
	def update_members(self,id,name,userlink,imagelink,designation):
		cur=self.cur
		self.id=id
		name_id=self.id
		self.name=name
		self.userlink=userlink
		self.imagelink=imagelink
		self.designation=designation
		cmd="UPDATE members set name='"
		cmd=cmd+self.name
		cmd=cmd+"',userlink='"
		cmd=cmd+self.userlink
		cmd=cmd+"',imagelink='"
		cmd=cmd+self.imagelink
		cmd=cmd+"',designation='"
		cmd=cmd+self.designation
		cmd=cmd+"' where name='"
		cmd=cmd+name_id+"';"
		cur.execute(cmd)
		self.conn.commit()
		return "Success"
	def insert_members(self,name,userlink,imagelink,designation):
		cur=self.cur
		self.name=name
		self.userlink=userlink
		self.imagelink=imagelink
		self.designation=designation
		cmd="INSERT into members values('"+self.name+"','"+self.userlink+"','"+self.imagelink+"','"+self.designation+"');"
		cur.execute(cmd)
		self.conn.commit()
		return "Success"
	def delete_members(self,name):
		cur=self.cur
		self.name=name
		cmd="DELETE from members where name='"+self.name+"';"
		cur.execute(cmd)
		self.conn.commit()
	def get_devices(self):
		cur=self.cur
		cur.execute("SELECT * FROM devices;")
		return cur.fetchall()
	def update_devices(self,id,name,link,code,dev,size):
		cur=self.cur
		self.id=id
		name_id=self.id
		self.name=name
		self.link=link
		self.code=code
		self.dev=dev
		self.size=size
		cmd="UPDATE devices set name='"+self.name+"',link='"+self.link+"',code='"+self.code+"',dev='"+self.dev+"',size='"+self.size+"' where code='"+name_id+"';"
		print(cmd)
		cur.execute(cmd)
		self.conn.commit()
		return "Success"
	def insert_devices(self,name,link,code,dev,size):
		cur=self.cur
		self.name=name
		self.link=link
		self.code=code
		self.dev=dev
		self.size=size
		cmd="INSERT into devices values('"+self.name+"','"+self.link+"','"+self.code+"','"+self.dev+"','"+self.size+"');"
		cur.execute(cmd)
		self.conn.commit()
		return "Success"
	def delete_devices(self,code):
		cur=self.cur
		self.code=code
		cmd="DELETE from devices where code='"+self.code+"';"
		cur.execute(cmd)
		self.conn.commit()
	def check_request(self,code):
		dev=self.get_devices()
		self.code=code
		for i in range(0,len(dev)):
			if self.code in dev[i]:
				return "dev_True"
				break
			else:
				continue
		cur=self.cur
		cur.execute("SELECT * from request;")
		req=cur.fetchall()
		for i in range(0,len(req)):
			if self.code in req[i]:
				number=int(req[i][1])+1
				cmd="UPDATE request set number="+"'"+str(number)+"'"+" where code="+"'"+req[i][0]+"';"
				cur.execute(cmd)
				self.conn.commit()
				return "req_True"
				break
			else:
				continue
		else:
			cmd="INSERT INTO request VALUES ("+"'"+self.code+"'"+",1);"
			cur.execute(cmd)
			self.conn.commit()
			return "req_False"
	def snd_req(self,name,code,tgu,kv,link,ver):
		cur=self.cur
		self.name=name
		self.code=code
		self.tgu=tgu
		self.kv=kv
		self.link=link
		self.ver=ver
		cur.execute("SELECT * from bot")
		bot=cur.fetchall()[0][0]
		txt="Device Name:"+self.name
		txt=txt+"\nDevice Codename:"+self.code
		txt=txt+"\nTelegram Username:@"+self.tgu
		txt=txt+"\nShipped Kernel Version:"+self.kv
		txt=txt+"\nSource Code Link:"+self.link
		txt=txt+"\nAndroid Version:"+self.ver
		url1="https://api.telegram.org/bot{}/".format(bot)
		url2="sendMessage?text={}&chat_id=-1001234510460".format(txt)
		url=url1+url2
		outp=requests.get(url)
