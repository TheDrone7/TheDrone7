import MinimalFooter from '../minimal/Footer.vue';

describe('Minimal Footer', () => {
  it('playground', () => {
    cy.mount(MinimalFooter);
  });

  it('renders properly', () => {
    cy.mount(MinimalFooter);
    cy.get('footer').should('exist');
  });
});
