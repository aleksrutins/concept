import LoginPage from './login'
import Counter from './lib/counter'
import logo from "./assets/imba.svg"

# Uncomment for dark mode:
# import './dark-styles'

css body
	ff:sans

tag concept-app
	<self>
		<div>

		<LoginPage route='/login'>

imba.mount <concept-app>, document.getElementById('app')
