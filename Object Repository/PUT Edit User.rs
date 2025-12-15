<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT Edit User</name>
   <tag></tag>
   <elementGuidId>11e99515-ee78-4d4e-a5e9-8154d0f6f14a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;job\&quot;: \&quot;${job}\&quot;,\n  \&quot;salary\&quot;: ${salary}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>200ceb17-afed-44b0-851f-684a66c8eda7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL}/users/${userId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>3761be41-8dc0-42a5-b304-5d1b9d707c7a</id>
      <name>JSON Schema</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\Purwo.Aji\OneDrive - PT IDX Solusi Teknologi Informasi\Documents\Bootcamp Katalon\schemagetuser.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>'12'</defaultValue>
      <description></description>
      <id>352e47b5-07dc-4819-ac83-c8cf43e353b8</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'Kepala Divisi'</defaultValue>
      <description></description>
      <id>213b6624-e464-4316-aae5-f27975e98bb8</id>
      <masked>false</masked>
      <name>job</name>
   </variables>
   <variables>
      <defaultValue>10000</defaultValue>
      <description></description>
      <id>deebf774-7506-4867-9bdf-2f2446b0fe7e</id>
      <masked>false</masked>
      <name>salary</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
