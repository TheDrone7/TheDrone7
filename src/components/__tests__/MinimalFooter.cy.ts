import MinimalFooter from '../minimal/Footer.vue';

describe('Minimal Footer', () => {
  it('playground', () => {
    cy.mount(MinimalFooter);
  });

  it('renders properly', () => {
    cy.mount(MinimalFooter);
    cy.get('footer').should('exist');
  });

  it('contains the copyright', () => {
    const currentYear = new Date().getFullYear();
    cy.mount(MinimalFooter);
    cy.get('footer').contains('© 2021').and('contain', currentYear.toString());
  });

  it('contains the link to socials', () => {
    cy.mount(MinimalFooter);
    cy.get('footer a').should('have.length', 4);
  });
});
