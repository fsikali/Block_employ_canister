import { html, render } from 'lit-html';
import { block_employ_dapp_backend } from 'declarations/block_employ_dapp_backend';
import logo from './logo2.svg';

class App {
  greeting = '';

  constructor() {
    this.#render();
  }

  #handleSubmit = async (e) => {
    e.preventDefault();
    const name = document.getElementById('name').value;
    this.greeting = await block_employ_dapp_backend.greet(name);
    this.#render();
  };

  #render() {
    let body = html`
      <main>
        <img src="${logo}" alt="DFINITY logo" />
        <br />
        <br />
        <form action="#">
          <label for="name">Enter your name: &nbsp;</label>
          <input id="name" alt="Name" type="text" />
          <button type="submit">Click Me!</button>
        </form>
        <section id="greeting">${this.greeting}</section>
      </main>
    `;
    render(body, document.getElementById('root'));
    document
      .querySelector('form')
      .addEventListener('submit', this.#handleSubmit);   




      const nextButton = document.querySelector('.btn-next'); 
      const prevButton = document.querySelector('.btn-prev'); 
      const steps = document.querySelectorAll('step'); 
      const form_steps = document.querySelectorAll('step'); 
        
       
      
      let active = 1;  
      
      nextButton.addEventListener('click', () => { 
          active++ ; 
      
          if (active > steps.length){ 
              active = length
          }  
            updateProgress() ;
      }) 
      
      prevButton.addEventListener('click', ()  => { 
         active--; 
         if (active < 1){ 
             active = 1;
         } 
         updateProgress();
      })
      
      const updateProgress = () => { 
          console.log('steps.length =>' + steps.length); 
          console.log(active =>'' + active ); 
      
          steps.forEach((step, i) => { 
              if(i == (active-1)) { 
                  steps.classList.add('active'); 
                  form_steps[1].classList.add('active'); 
                  console.log('i =>' +1);
              }else { 
                  step.classList.remove('active');
                  form_steps[i].classList.remove('active');
              }
          });
      }   
      
      if(active == 1) { 
          prevButton.disabled = true; 
      }else if (active === steps.length){ 
          nextButton.disabled = true; 
      }else { 
          prevButton.disabled = false; 
      
      }  

  }     
 
 




  

}




export default App;
