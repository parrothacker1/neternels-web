from modules.connector import SQL
import base64,os
from flask import Flask,render_template
from flask_heroku import Heroku
from flask import request as req
from flask_mobility import Mobility
app=Flask(__name__)
Mobility(app)
Heroku(app)
sql=SQL()
@app.route('/')
def index():
	return render_template('index.html',name=sql.get_members(),len1=len(sql.get_members()),dev=sql.get_devices(),len2=len(sql.get_devices()))

@app.route('/download')
def download():
	return render_template('downloads.html',name=sql.get_devices(),len=len(sql.get_devices()))

@app.route('/kernel')
def kernel():
	return render_template('request.html')

@app.route('/request')
def request():
	if req.method=="GET":
		name=req.args.get('name','')
		code=req.args.get('code','')
		tgu=req.args.get('tgu','')
		kv=req.args.get('kv','')
		link=req.args.get('link','')
		ver=req.args.get('ver','')
		res=sql.check_request(code)
		if res == "req_False":
			sql.snd_req(name,code,tgu,kv,link,ver)
		else:
			None
		return render_template('req_return.html',outp=res)
	else:
		None

@app.route('/howto')
def howto():
	return render_template('howto.html')

@app.route('/dev')
def dev():
	return render_template('dev_login.html')

@app.route('/dev_login')
def login():
	if req.method=="GET":
		mmbr=req.args.get('mmbr','')
		passwd=req.args.get('passwd','')
		passwd_decoded=base64.b64decode(passwd)
		for x in sql.get_members():
 			if mmbr in x and passwd_decoded.decode()==sql.passwd():
 				sql.login("True")
 				break
 			else:
 				sql.login("False")
 				continue
		if "True" in sql.ret_log()[0]:
			return render_template('dev_edit.html',devices=sql.get_devices(),members=sql.get_members(),len_mem=len(sql.get_members()),len_dev=len(sql.get_devices()))
		else:

			return "Fuck off skidd"
	else:
		return "FAILED TO LOAD"
@app.route('/dev_update/<id1>/<id2>')
def update(id1,id2):
	if "True" in sql.ret_log()[0]:
		if id1=="member":
			if id2=="update":
				if req.method=="GET":
					id=req.args.get('id')
					name=req.args.get('name')
					usrlnk=req.args.get('usrlnk')
					imglnk=req.args.get('imglnk')
					desig=req.args.get('desig')
					sql.update_members(id,name,usrlnk,imglnk,desig)
					return "Done"
				else:
					return "Fail"
			if id2=="insert":
				if req.method=="GET":
					name=req.args.get('name')
					usrlnk=req.args.get('usrlnk')
					imglnk=req.args.get('imglnk')
					desig=req.args.get('desig')
					sql.insert_members(name,usrlnk,imglnk,desig)
					return "DONE"
				else:
					return "Fail"
		elif id1=="device":
			if id2=="update":
				if req.method=="GET":
					id=req.args.get('id')
					name=req.args.get('name')
					code=req.args.get('code')
					link=req.args.get('lnk')
					dev=req.args.get('dev')
					size=req.args.get('size')
					sql.update_devices(id,name,link,code,dev,size)
					print(sql.get_devices())
					return "Done"
				else:
					return "Nill"
			if id2=="insert":
				if req.method=="GET":
					name=req.args.get('name')
					code=req.args.get('code')
					link=req.args.get('lnk')
					dev=req.args.get('dev')
					size=req.args.get('size')
					for i in range(0,len(sql.get_devices())):
						if code not in sql.get_devices()[i]:
							sql.insert_devices(name,link,code,dev,size)
							return "DoNe"
						else:
							return "Repetition"
							break
				else:
					return "Nill"
		else:
			return "Check the link"
	else:
		return "Fuck off skidd"
@app.route('/delete/<id1>/<id2>')
def dele(id1,id2):
	if "True" in sql.ret_log()[0]:
		if id1=="devices":
			sql.delete_devices(id2)
			return "Done"
		else:
			None
	else:
		return "Fuck off skid"
@app.route('/dev_logout')
def logout():
	sql.login("False")
	return "<script>alert('Bye');window.close();</script>"
@app.errorhandler(404)
def not_found(e):
	return render_template('404.html')
if __name__=='__main__':
	app.config['TEMPLATES_AUTO_RELOAD'] = True
	app.config['SEND_FILE_MAX_AGE_DEFAULT'] = 0
	app.run(host='0.0.0.0',debug=True,threaded=True)

