// https://on.cypress.io/api

describe('Minimal view experience', () => {
  it('visits the app root url', () => {
    cy.visit('/');
    cy.contains('h1', 'Harmeet Singh Bhatia');
  });

  it('loads all components', () => {
    cy.visit('/');
    cy.get('header').should('exist');
    cy.get('h2').contains('Info').should('exist');
    cy.get('h2').contains('Previous work').should('exist');
    cy.get('h2').contains('Other projects').should('exist');
    cy.get('footer').should('exist');
  });

  it('can switch to light mode', () => {
    cy.visit('/');
    cy.get('html').should('have.class', 'dark');
    cy.get('button[fab]').click();
    cy.get('html').should('not.have.class', 'dark');
    cy.get('button[fab]').click();
    cy.get('html').should('have.class', 'dark');
  });
});
