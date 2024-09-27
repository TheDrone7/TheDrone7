import MinimalHeader from '../minimal/Header.vue';

describe('Minimal Header', () => {
  it('playground', () => {
    cy.mount(MinimalHeader);
  });

  it('renders properly', () => {
    cy.mount(MinimalHeader);
    cy.get('header').should('exist');
  });

  it('contains the name(s)', () => {
    cy.mount(MinimalHeader);
    cy.get('header').contains('Harry Baines').and('contain', 'TheDrone7');
  });

  it('contains the link to email', () => {
    cy.mount(MinimalHeader);
    cy.get('header a').should('have.length', 1).and('have.attr', 'href', 'mailto:h@thedrone7.dev');
  });
});
