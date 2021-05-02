#from modules.connector import SQL
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
        return render_template('index.html',name=sql.get_members(),len1=len(sql.get_mem>
if __name__=="__main__":
	app.run()
