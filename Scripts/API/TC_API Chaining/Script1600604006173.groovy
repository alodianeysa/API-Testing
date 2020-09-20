import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('API/GET_List_User'))

def slurper = new groovy.json.JsonSlurper()
//mendefinisikan objek slurper agar mengganti bahasa groofy Json 

def result = slurper.parseText(response.getResponseBodyContent())
//mengubah ke parseText, menggunakan fungsi Get

def value_firstName = result.data[0].first_name
//memanggil verificarion 

def value_lastName = result.data[0].last_name

println('value is ' + value_firstName)

println('value is ' + value_lastName)

GlobalVariable.name = value_firstName
//panggilan menggunakan globalVariable
println('Update value is ' + GlobalVariable.name)
//menimpa value name yang ada pada globalVariable

WS.sendRequestAndVerify(findTestObject('API/PUT_Update'))
//sehingga output yang keluar menggunakan timpaan pada sblmnya
//verification di ListUser juga bisa menggunakan update timpaan

