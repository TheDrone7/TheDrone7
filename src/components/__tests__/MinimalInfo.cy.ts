import MinimalInfo from '../minimal/Info.vue';

describe('Minimal Info', () => {
  it('playground', () => {
    cy.mount(MinimalInfo);
  });

  it('renders properly', () => {
    cy.mount(MinimalInfo);
    cy.get('section').should('exist');
    cy.get('h2').should('have.text', 'Info');
  });
});
