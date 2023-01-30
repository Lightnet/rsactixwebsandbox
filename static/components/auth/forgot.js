
import {
  createSignal,
  onCleanup,
} from "https://cdn.skypack.dev/solid-js";
import h from "https://cdn.skypack.dev/solid-js/h";

function Forgot(props){

  const [alias, setAlias] = createSignal("test");
  const [passphrase, setPassphrase] = createSignal("test");
  const [passphrase2, setPassphrase2] = createSignal("test");

  function InputAlias(e){
    setAlias(e.target.value);
  }

  function InputPassphrase(e){
    setPassphrase(e.target.value);
  }

  function InputPassphrase2(e){
    setPassphrase2(e.target.value);
  }

  function btnRecovery(){
    console.log("btnRecovery...")
    //if(typeof props.view == 'function'){
      //props.view('signup')
    //}
  }
  function btnBack(){
    console.log("btnBack...")
    if(typeof props.view == 'function'){
      props.view('signin')
    }
  }


  return h("table",{}, h("tbody",{},
    h("tr",{},
      h("td",{colspan:2},
        h("label",{},"Forgot"),
      ),
    ),
    h("tr",{},
      h("td",{},h("label",{},"Alias:")),
      h("td",{},h("input",{value:alias(),onInput:InputAlias})),
    ),
    h("tr",{},
      h("td",{},h("label",{},"Passphrase:")),
      h("td",{},h("input",{value:passphrase(),onInput:InputPassphrase})),
    ),
    h("tr",{},
      h("td",{},h("label",{},"Verify Passphrase:")),
      h("td",{},h("input",{value:passphrase2(),onInput:InputPassphrase2})),
    ),
    h("tr",{},
      h("td",{colspan:2},
        h("button",{onClick:btnBack},"Back"),
        h("button",{onClick:btnRecovery},"Recovery"),
      ),
    ),
  ))
}

export default Forgot;